/// Enum for different kind of timers related to otp
pub enum OtpTimers<'a> {
    Otp(&'a str),
    OtpBan(&'a str),
    OtpCooldown(&'a str),
    OtpRequests(&'a str),
    OtpAttempts(&'a str),
}

impl<'a> OtpTimers<'a> {
    /// Centralised function to generate keys for otp timers
    pub fn key(&self) -> String {
        match self {
            OtpTimers::Otp(phone) => format!("otp:{}", phone),
            OtpTimers::OtpBan(phone) => format!("otp_ban:{}", phone),
            OtpTimers::OtpCooldown(phone) => format!("otp_cooldown:{}", phone),
            OtpTimers::OtpRequests(phone) => format!("otp_requests:{}", phone),
            OtpTimers::OtpAttempts(phone) => format!("otp_attempts:{}", phone),
        }
    }
}
