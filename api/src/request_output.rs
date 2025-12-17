use serde::{Deserialize, Serialize};
use store::models::website::{AvgRespTime, DailyView, HourlyView, MinuteView, TotalUniqueUsers, TotalViews, TotalViewsPerPage, UptimePercentage};

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
pub struct GetViewsPerPageOutput {
    pub data: Option<Vec<TotalViewsPerPage>>,
    pub success: bool
}

#[derive(Serialize, Deserialize)]
pub struct GetTotalUniqueUsersOutput {
    pub data: Option<TotalUniqueUsers>,
    pub success: bool
}

#[derive(Serialize, Deserialize)]
pub struct GetTotalViewsOutput {
    pub data: Option<TotalViews>,
    pub success: bool
}

#[derive(Serialize, Deserialize)]
pub struct GetWebsiteDetailsLastHourOutput {
    pub data: Option<Vec<MinuteView>>,
    pub success: bool
}

#[derive(Serialize, Deserialize)]
pub struct GetWebsiteAvgRespTimeOutput {
    pub data: Option<AvgRespTime>,
    pub success: bool
}

#[derive(Serialize, Deserialize)]
pub struct GetUptimePercentageOutput {
    pub data: Option<UptimePercentage>,
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

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub plan_type: String,
    pub success: bool
}