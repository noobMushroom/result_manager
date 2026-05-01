use redis::{AsyncCommands, RedisError, aio::ConnectionManager};
use secrecy::{ExposeSecret, SecretString};

use crate::{domain::phone::Phone, errors::AppError, otp::error::OtpErrors, repostiory::utils::OtpTimers};

/// Otp repo struct store redis connection and function to run db methods following repository
/// pattern
#[derive(Clone)]
pub struct OtpRepo {
    pub con: ConnectionManager,
}

impl OtpRepo {
    /// Creates a new ConnectionManager and returns a OtpRepo struct
    pub async fn new(connection_string: &str) -> Result<Self, RedisError> {
        let client = redis::Client::open(connection_string)?;
        let con = ConnectionManager::new(client).await?;
        Ok(Self { con })
    }

    /// Gets the otp form db if otp not found returns OtpExpiredOrNotFound 
    #[tracing::instrument(name = "Gets the otp from db", skip(self), fields(phone = %phone.as_ref()))]
    pub async fn get_otp(
        &self,
        phone: &Phone,
    ) -> Result<String, AppError> {
        let mut con = self.con.clone();
        let key = OtpTimers::Otp(phone.as_ref()).key();
        let otp: Option<String> = con.get(&key)
            .await
            .map_err(|e| {
                tracing::error!(error=?e, "Failed to get otp hash from the db");
                AppError::Internal
            })?;

        let otp = otp.ok_or(OtpErrors::OtpExpiredOrNotFound("Otp expired or not found try again".into()))?;
        Ok(otp)
    }

    ///Saves the otp hash with ttl into the database
    #[tracing::instrument(name = "Inserting the otp into the database", skip(otp_hash, self), fields(phone = %phone.as_ref(), ttl = %ttl))]
    pub async fn insert_otp(
        &self,
        otp_hash: SecretString,
        phone: &Phone,
        ttl: u64,
    ) -> Result<(), AppError> {
        let mut con = self.con.clone();
        let key = OtpTimers::Otp(phone.as_ref()).key();
        con.set_ex::<_, _, ()>(key, otp_hash.expose_secret(), ttl)
            .await
            .map_err(|e| {
                tracing::error!(error=?e, "Failed to insert the otp hash in the db");
                AppError::Internal
            })?;
        Ok(())
    }

    ///Sets the cooldown time after user requested otp
    #[tracing::instrument(name = "Inserting the phone for cooldown", skip(self), fields(phone = %phone.as_ref(), ttl = %ttl))]
    pub async fn set_cooldown(&self, phone: &Phone, ttl: u64) -> Result<(), AppError> {
        let mut con = self.con.clone();
        let key = OtpTimers::OtpCooldown(phone.as_ref()).key();

        let result: Option<String> = con
            .set_options(
                &key,
                1,
                redis::SetOptions::default()
                    .conditional_set(redis::ExistenceCheck::NX)
                    .with_expiration(redis::SetExpiry::EX(ttl)),
            )
            .await
            .map_err(|e| {
                tracing::error!(error=?e, "Failed to insert the cooldown in db");
                AppError::Internal
            })?;

        if result.is_none() {
            return Err(AppError::TooManyRequests(
                "Wait 30 seconds before requesting for otp again".into(),
            ));
        }

        Ok(())
    }

    /// Bans the user for  the given ttl time
    #[tracing::instrument(name = "Inserting the user to ban", skip(self), fields(phone = %phone.as_ref(), ttl = %ttl))]
    pub async fn ban_user(&self, phone: &Phone, ttl: u64) -> Result<(), AppError> {
        let mut con = self.con.clone();
        let key = OtpTimers::OtpBan(phone.as_ref()).key();
        con.set_ex::<_, _, ()>(key, 1, ttl).await.map_err(|e| {
            tracing::error!(error=?e, "Failed to insert the ban db");
            AppError::Internal
        })?;
        Ok(())
    }

    /// Increment the number of time user requested for the otp and sets timer on the first time for
    /// 5 mins, if requested otp for more than 3 times then bans the user
    #[tracing::instrument(name = "Increasing the otp requests attempt and if it's more than 3 adding to the ban table", skip(self), fields(phone = %phone.as_ref(), ttl = %ttl))]
    pub async fn increment_otp_requests(&self, phone: &Phone, ttl: u64) -> Result<(), AppError> {
        let mut con = self.con.clone();
        let key = OtpTimers::OtpRequests(phone.as_ref()).key();

        let attempts: u64 = con.incr(&key, 1).await.map_err(|e| {
            tracing::error!(error=?e, "Failed to increment otp request attempts");
            AppError::Internal
        })?;

        if attempts == 1 {
            let _: () = con.expire(&key, 300).await.map_err(|e| {
                tracing::error!(error=?e, "Failed to set expiry for otp requests");
                AppError::Internal
            })?;
        }

        if attempts == 4 {
            self.ban_user(phone, ttl).await?;
        }

        Ok(())
    }

    /// Increment the number of time user entered the wrong attempts and sets timer for 10 mins on
    /// the first try, and if tried wrong otp for more than 5 times bans the user
    #[tracing::instrument(name = "Increasing the otp wrong atteempts if more than 5 bans the user", skip(self), fields(phone = %phone.as_ref(), ttl = %ttl))]
    pub async fn increment_wrong_attempts(&self, phone: &Phone, ttl: u64) -> Result<(), AppError> {
        let mut con = self.con.clone();
        let key = OtpTimers::OtpAttempts(phone.as_ref()).key();

        let attempts: u64 = con.incr(&key, 1).await.map_err(|e| {
            tracing::error!(error=?e, "Failed to increment otp wrong attempts");
            AppError::Internal
        })?;

        if attempts == 1 {
            let _: () = con.expire(&key, 600).await.map_err(|e| {
                tracing::error!(error=?e, "Failed to set expiry for wrong otp attempts");
                AppError::Internal
            })?;
        }

        if attempts == 6 {
            self.ban_user(phone, ttl).await?;
        }

        Ok(())
    }

    /// Checks if the user is in ban table if yes then return forbidden status code
    #[tracing::instrument(name = "Checking if user is banned", skip(self), fields(phone = %phone.as_ref()))]
    pub async fn ensure_not_banned(&self, phone: &Phone) -> Result<(), AppError> {
        let mut con = self.con.clone();
        let user = con
            .exists(&OtpTimers::OtpBan(phone.as_ref()).key())
            .await
            .map_err(|e| {
                tracing::error!(error=?e, "Failed to fetch user info from the table");
                AppError::Internal
            })?;

        if user {
            return Err(AppError::Forbidden);
        }

        Ok(())
    }

    /// Clears all the timers from db (otp_cooldown:<PHONE>, wrond_attempts:<PHONE>,
    /// otp_requests:<PHONE>, ban:<PHONE>)
    #[tracing::instrument(name = "Checking if user is banned", skip(self), fields(phone = %phone.as_ref()))]
    pub async fn reset_bans(&self, phone: &Phone) -> Result<(), AppError> {
        let mut con = self.con.clone();

        let keys = [
            OtpTimers::OtpCooldown(phone.as_ref()).key(),
            OtpTimers::OtpBan(phone.as_ref()).key(),
            OtpTimers::OtpRequests(phone.as_ref()).key(),
            OtpTimers::OtpAttempts(phone.as_ref()).key(),
            OtpTimers::Otp(phone.as_ref()).key(),
        ];

        let _: () = con.del(&keys).await.map_err(|e| {
            tracing::error!(error=?e, "Failed to delete keys from the db");
            AppError::Internal
        })?;

        Ok(())
    }
}
