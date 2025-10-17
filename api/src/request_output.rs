use serde::{Deserialize, Serialize};
use store::models::website::{DailyView, HourlyView};

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
pub struct GetWebsiteDetailsHourlyOutput {
    pub data: Option<Vec<HourlyView>>,
    pub success: bool
}

#[derive(Serialize, Deserialize)]
pub struct GetWebsiteDetailsDailyOutput {
    pub data: Option<Vec<DailyView>>,
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
    pub plan_type: String,
    pub success: bool
}