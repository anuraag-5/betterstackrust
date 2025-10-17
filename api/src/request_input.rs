use serde::{Deserialize, Serialize};
use store::models::website::Website;

#[derive(Serialize, Deserialize)]
pub struct CreateWebsiteInput {
    pub url: String,
    pub about: String,
    pub user_id: String
}

#[derive(Deserialize, Serialize)]
pub struct CreateUserInput {
    pub username: String,
    pub password: String,
    pub name: String
}

#[derive(Deserialize, Serialize)]
pub struct SignInUserInput {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct TrackingInput {
    pub visitor_id: String,
    pub page_url: String,
    pub referrer: String,
    pub user_agent: String,
    pub time_stamp: String
}

#[derive(Deserialize, Serialize)]
pub struct UsersWebsites {
    pub websites: Option<Vec<Website>>,
    pub success: bool
}

#[derive(Deserialize, Serialize)]
pub struct GetWebsiteDetailsDailyInput {
    pub user_id: String,
    pub website: String,
    pub day:  String
}

#[derive(Deserialize, Serialize)]
pub struct GetWebsiteDetailsHourlyInput {
    pub user_id: String,
    pub website: String,
    pub hour:  String
}