use reqwest::{header};
use reqwest::blocking::{Client, Response};
use crate::radar::error::RadarError;

pub const CF_API_BASE_URL: &str = "https://api.cloudflare.com/client/v4";

pub struct RadarClient {
    client: Client,
}

impl RadarClient {
    pub fn new() -> Result<Self, RadarError> {
        // load dot env files
        dotenvy::dotenv().ok();

        let api_token = match std::env::var("CF_API_TOKEN") {
            Ok(token) => token,
            Err(_) => return Err(RadarError::TokenError("missing environment variable CF_API_TOKEN".to_string())),
        };

        // build reqwest client with default authorization token
        let mut headers = header::HeaderMap::new();
        headers.insert("Authorization", header::HeaderValue::from_str(format!("Bearer {}", api_token).as_str()).unwrap());
        headers.insert("Content-Type", header::HeaderValue::from_static("application/json"));
        let client = reqwest::blocking::ClientBuilder::new()
            .user_agent("radar-rs/0.1")
            .default_headers(headers)
            .build()?;

        Self::verify_token(&client)?;

        Ok(Self { client })
    }

    fn verify_token(client: &Client) -> Result<(), RadarError> {
        let response = client.get(format!("{}/user/tokens/verify", CF_API_BASE_URL)).send()?;
        if !response.status().is_success() {
            return Err(RadarError::TokenError("invalid api token".to_string()));
        }
        Ok(())
    }

    pub fn send_request(&self, route: &str) -> Result<Response, RadarError> {
        let url = format!("{}/{}", CF_API_BASE_URL, route);
        Ok(self.client.get(url).send()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_validation() {
        // NOTE: need to set a valid CF_API_TOKEN in .env file
        RadarClient::new().unwrap();
    }
}