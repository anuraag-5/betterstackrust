use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateWebsiteInput {
    pub url: String,
}


#[derive(Deserialize, Serialize)]
pub struct CreateUserInput {
    pub username: String,
    pub password: String
}
