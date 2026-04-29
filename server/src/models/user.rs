use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub office_location: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_phones: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct GraphResponse<T> {
    pub value: Vec<T>,
}
