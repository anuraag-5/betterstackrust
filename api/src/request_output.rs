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
