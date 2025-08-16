use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateWebsiteOutput {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserOutput {
    pub jwt: String,
    pub success: bool
}
