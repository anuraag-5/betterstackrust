use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateWebsiteOutput {
    pub website_id: String,
    pub success: bool
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserOutput {
    pub user_id: String,
    pub success: bool
}

#[derive(Serialize, Deserialize)]
pub struct SigninUserOutput {
    pub jwt: String,
    pub success: bool
}

#[derive(Serialize, Deserialize)]
pub struct GetWebsiteOutput {
    pub url: String,
    pub success: bool
}

#[derive(Serialize, Deserialize)]
pub struct TotalViewsOutput {
    pub total_views: i64,
    pub success: bool
}

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub success: bool
}