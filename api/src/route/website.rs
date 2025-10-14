use std::sync::{Arc, Mutex};

use crate::{
    auth_middleware::UserIdFromHeader,
    request_input::{ CreateWebsiteInput, UsersWebsites, GetWebsiteDetailsInput },
    request_output::{ CreateWebsiteOutput, GetWebsiteDetailsHourlyOutput },
};
use poem::{
    handler,
    web::{Data, Json},
};
use store::{store::Store};

#[handler]
pub fn create_website(
    Json(data): Json<CreateWebsiteInput>,
    Data(s): Data<&Arc<Mutex<Store>>>
) -> Json<CreateWebsiteOutput> {
    let url = data.url;
    let about = data.about;
    let user_id= data.user_id;
    let mut locked_s = s.lock().unwrap();
    let created_website = locked_s.create_website(user_id, url, about);
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
pub fn get_details_hourly(
    Data(s): Data<&Arc<Mutex<Store>>>,
    Json(data): Json<GetWebsiteDetailsInput>
) -> Json<GetWebsiteDetailsHourlyOutput> {
    let mut locked_s = s.lock().unwrap();
    let website_result = locked_s.get_website_details(data.website, data.user_id);
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
pub fn get_users_websites(
    Data(s): Data<&Arc<Mutex<Store>>>,
    UserIdFromHeader(user_id): UserIdFromHeader
) -> Json<UsersWebsites> {
    let mut locked_s = s.lock().unwrap();
    let res = locked_s.get_users_all_websites(user_id);

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