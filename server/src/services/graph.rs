use crate::models::user::{GraphResponse, UserProfile};
use log::warn;
use reqwest::Client;

pub struct GraphService;

impl GraphService {
    const BASE_URL: &'static str = "https://graph.microsoft.com/v1.0";

    pub async fn fetch_profile(token: &str) -> Result<UserProfile, reqwest::Error> {
        let client = Client::new();
        client
            .get(format!("{}/me", Self::BASE_URL))
            .bearer_auth(token)
            .send()
            .await?
            .error_for_status()?
            .json::<UserProfile>()
            .await
    }

    pub async fn fetch_all_users(token: &str) -> Result<Vec<UserProfile>, reqwest::Error> {
        let client = Client::new();
        let res = client
            .get(format!("{}/users", Self::BASE_URL))
            .bearer_auth(token)
            .send()
            .await?
            .error_for_status()?;

        let data = res.json::<GraphResponse<UserProfile>>().await?;
        Ok(data.value)
    }

    pub async fn update_profile(
        token: &str,
        profile: &UserProfile,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();
        let response = client
            .patch(format!("{}/me", Self::BASE_URL))
            .bearer_auth(token)
            .json(profile)
            .send()
            .await?;

        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let body = response.text().await.unwrap_or_default();
            warn!("--- GRAPH UPDATE FAILED ---");
            warn!("Status: {}", status);
            warn!("Message: {}", body);

            let error_msg = format!("Graph API Error ({}): {}", status, body);
            Err(error_msg.into())
        }
    }
}
