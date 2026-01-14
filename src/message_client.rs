use reqwest::Client;
use secrecy::{ExposeSecret, SecretString};

use crate::domain::phone::Phone;

pub struct MessageClient {
    http_client: Client,
    base_url: String,
    auth_id: SecretString,
    device_id: SecretString,
}

#[derive(serde::Serialize)]
struct SendMessageRequest {
    recipients: Vec<String>,
    message: String,
}

impl MessageClient {
    pub fn new(
        timeout: std::time::Duration,
        base_url: String,
        auth_id: SecretString,
        device_id: SecretString,
    ) -> Result<Self, reqwest::Error> {
        let client = Client::builder().timeout(timeout).build()?;

        Ok(Self {
            http_client: client,
            base_url,
            auth_id,
            device_id,
        })
    }

    pub async fn send_otp(&self, otp: SecretString, phone: &Phone) -> Result<(), reqwest::Error> {
        let message = format!(
            "Your login otp for St James Result is {}",
            otp.expose_secret()
        );

        let url = format!(
            "{}/{}/send-sms",
            self.base_url,
            self.device_id.expose_secret()
        );
        let request_body = SendMessageRequest {
            recipients: vec![phone.as_ref().to_string()],
            message,
        };

        println!("hrere");

        self.http_client
            .post(&url)
            .header("X-api-Key", self.auth_id.expose_secret())
            .json(&request_body)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{header, method},
    };

    use super::*;

    struct MessageClientBodyMatcher;

    impl wiremock::Match for MessageClientBodyMatcher {
        fn matches(&self, request: &wiremock::Request) -> bool {
            let result: Result<serde_json::Value, _> = serde_json::from_slice(&request.body);
            if let Ok(body) = result {
                body.get("recipients").is_some() && body.get("message").is_some()
            } else {
                false
            }
        }
    }

    fn recipients() -> Phone {
        Phone::parse("1234567890").unwrap()
    }

    fn message_client(base_url: String) -> MessageClient {
        MessageClient::new(
            std::time::Duration::from_millis(300),
            base_url,
            SecretString::new("2144222222".into()),
            SecretString::new("2144222222".into()),
        )
        .unwrap()
    }

    fn otp() -> SecretString {
        SecretString::new("aua".into())
    }

    #[actix::test]
    async fn send_message_fires_request_to_base_url() {
        let mock_server = MockServer::start().await;

        let message_client = message_client(mock_server.uri());

        Mock::given(header("Content-Type", "application/json"))
            .and(method("POST"))
            .and(MessageClientBodyMatcher)
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = message_client.send_otp(otp(), &recipients()).await;
    }

    #[actix::test]
    async fn send_message_succeeds_if_the_server_returns_200() {
        let mock_server = MockServer::start().await;
        let message_client = message_client(mock_server.uri());

        Mock::given(header("Content-Type", "application/json"))
            .and(method("POST"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let respond = message_client.send_otp(otp(), &recipients()).await;

        assert!(respond.is_ok())
    }

    #[actix::test]
    async fn send_message_fails_if_the_server_returns_500() {
        let mock_server = MockServer::start().await;
        let message_client = message_client(mock_server.uri());

        Mock::given(header("Content-Type", "application/json"))
            .and(method("POST"))
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&mock_server)
            .await;

        let respond = message_client.send_otp(otp(), &recipients()).await;

        assert!(respond.is_err())
    }

    #[actix::test]
    async fn send_message_fails_if_the_server_timesout() {
        let mock_server = MockServer::start().await;
        let message_client = message_client(mock_server.uri());
        let respones = ResponseTemplate::new(200).set_delay(std::time::Duration::from_millis(400));

        Mock::given(header("Content-Type", "application/json"))
            .and(method("POST"))
            .respond_with(respones)
            .expect(1)
            .mount(&mock_server)
            .await;

        let respond = message_client.send_otp(otp(), &recipients()).await;

        assert!(respond.is_err())
    }
}
