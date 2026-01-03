use std::sync::{Arc};

use crate::{
    auth_middleware::UserIdFromHeader,
    request_input::{ CreateWebsiteInput, GetUptimePercentage, GetUptimePercentageByRegion, GetWebsiteAverageRespTime, GetWebsiteAverageRespTimeByRegion, GetWebsiteDetailsDailyInput, GetWebsiteDetailsHourlyInput, GetWebsiteDetailsLastHourInput, UsersWebsites },
    request_output::{ CreateWebsiteOutput, GetUptimePercentageOutput, GetWebsiteAvgRespTimeOutput, GetWebsiteDetailsDailyOutput, GetWebsiteDetailsHourlyOutput, GetWebsiteDetailsLastHourOutput },
};
use poem::{
    handler,
    web::{Data, Json},
};
use store::{models::website::Status, store::Store};

#[handler]
pub async fn create_website(
    Json(data): Json<CreateWebsiteInput>,
    Data(s): Data<&Arc<Store>>
) -> Json<CreateWebsiteOutput> {
    let url = data.url;
    let about = data.about;
    let user_id= data.user_id;
    
    let created_website = s.create_website(user_id, url, about).await;
    match created_website {
        Ok(w) => Json(CreateWebsiteOutput {
            website_id: w.id,
            success: true,
        }),
        Err(e) => Json(CreateWebsiteOutput {
            website_id: e.to_string(),
            success: false,
        }),
    }
}

#[handler]
pub async fn get_website_recent_status(
    Data(s): Data<&Arc<Store>>,
    Json(data): Json<GetWebsiteDetailsLastHourInput>
) -> Json<Status> {
    
    let website_result = s.get_website_recent_status(data.website, data.user_id).await;
    match website_result {
        Ok(s) => Json(s),
        Err(_) => Json(Status{
            status: "Unknown".into()
        }),
    }
}

#[handler]
pub async fn get_details_hourly(
    Data(s): Data<&Arc<Store>>,
    Json(data): Json<GetWebsiteDetailsHourlyInput>
) -> Json<GetWebsiteDetailsHourlyOutput> {
    
    let website_result = s.get_website_details_hourly(data.website, data.user_id, data.hour).await;
    match website_result {
        Ok(w) => Json(GetWebsiteDetailsHourlyOutput {
            data: Some(w),
            success: true,
        }),
        Err(_) => Json(GetWebsiteDetailsHourlyOutput {
            data: None,
            success: false,
        }),
    }
}

#[handler]
pub async fn get_details_daily(
    Data(s): Data<&Arc<Store>>,
    Json(data): Json<GetWebsiteDetailsDailyInput>
) -> Json<GetWebsiteDetailsDailyOutput> {
    
    let website_result = s.get_website_details_daily(data.website, data.user_id, data.day).await;
    match website_result {
        Ok(w) => Json(GetWebsiteDetailsDailyOutput {
            data: Some(w),
            success: true,
        }),
        Err(_) => Json(GetWebsiteDetailsDailyOutput {
            data: None,
            success: false,
        }),
    }
}

#[handler]
pub async fn get_details_last_hour(
    Data(s): Data<&Arc<Store>>,
    Json(data): Json<GetWebsiteDetailsLastHourInput>
) -> Json<GetWebsiteDetailsLastHourOutput> {
    
    let website_result = s.get_website_details_last_hour(data.website, data.user_id).await;
    match website_result {
        Ok(w) => Json(GetWebsiteDetailsLastHourOutput {
            data: Some(w),
            success: true,
        }),
        Err(_) => Json(GetWebsiteDetailsLastHourOutput {
            data: None,
            success: false,
        }),
    }
}

#[handler]
pub async fn get_users_websites(
    Data(s): Data<&Arc<Store>>,
    UserIdFromHeader(user_id): UserIdFromHeader
) -> Json<UsersWebsites> {
    
    let res = s.get_users_all_websites(user_id).await;

    match res {
        Ok(websites) => {
            let users_websites = UsersWebsites { websites: Some(websites), success: true };
            Json(users_websites)
        }
        Err(_) => {
            let users_websites = UsersWebsites { websites: None, success: false };
            Json(users_websites)
        }
    }
}

#[handler]

pub async fn get_avg_resp(
    Data(s): Data<&Arc<Store>>,
    Json(data): Json<GetWebsiteAverageRespTime>
) ->  Json<GetWebsiteAvgRespTimeOutput> {
    
    let input_website = data.website;
    let res = s.get_average_resp_time(input_website).await;

    match res {
        Ok(avg) => {
            Json(GetWebsiteAvgRespTimeOutput {
                data: Some(avg),
                success: true
            })
        },
        Err(e) => {
            println!("Error: {}", e);
            Json(GetWebsiteAvgRespTimeOutput {
                data: None,
                success: false
            }) 
        }
    }
}

#[handler]

pub async fn get_avg_resp_by_region(
    Data(s): Data<&Arc<Store>>,
    Json(data): Json<GetWebsiteAverageRespTimeByRegion>
) ->  Json<GetWebsiteAvgRespTimeOutput> {
    
    let input_website = data.website;
    let input_region = data.region;
    let res = s.get_average_resp_time_by_region(input_website, input_region).await;

    match res {
        Ok(avg) => {
            Json(GetWebsiteAvgRespTimeOutput {
                data: Some(avg),
                success: true
            })
        },
        Err(e) => {
            println!("Error: {}", e);
            Json(GetWebsiteAvgRespTimeOutput {
                data: None,
                success: false
            }) 
        }
    }
}

#[handler]
pub async fn get_uptime_percentage(
    Data(s): Data<&Arc<Store>>,
    Json(data): Json<GetUptimePercentage>
) ->  Json<GetUptimePercentageOutput> {
    
    let input_website = data.website;
    let res = s.get_average_uptime_percentage(input_website).await;

    match res {
        Ok(up) => {
            Json(GetUptimePercentageOutput {
                data: Some(up),
                success: true
            })
        },
        Err(e) => {
            println!("Error: {}", e);
            Json(GetUptimePercentageOutput {
                data: None,
                success: false
            }) 
        }
    }
}

#[handler]
pub async fn get_uptime_percentage_by_region(
    Data(s): Data<&Arc<Store>>,
    Json(data): Json<GetUptimePercentageByRegion>
) ->  Json<GetUptimePercentageOutput> {    
    let input_website = data.website;
    let input_region = data.region;
    let res = s.get_average_uptime_percentage_by_region(input_website, input_region).await;

    match res {
        Ok(up) => {
            Json(GetUptimePercentageOutput {
                data: Some(up),
                success: true
            })
        },
        Err(e) => {
            println!("Error: {}", e);
            Json(GetUptimePercentageOutput {
                data: None,
                success: false
            }) 
        }
    }
}