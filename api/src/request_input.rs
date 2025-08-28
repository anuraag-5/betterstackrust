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

#[derive(Deserialize, Serialize)]
pub struct TrackingInput {
    pub visitor_id: String,
    pub page_url: String,
    pub referrer: String,
    pub user_agent: String,
    pub time_stamp: String
}