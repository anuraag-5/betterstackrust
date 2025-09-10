use std::sync::{Arc, Mutex};

use crate::{
    auth_middleware::UserId,
    request_input::CreateWebsiteInput,
    request_output::{ CreateWebsiteOutput, GetWebsiteOutput },
};
use poem::{
    handler,
    web::{Data, Json, Path},
};
use store::store::Store;

#[handler]
pub fn create_website(
    Json(data): Json<CreateWebsiteInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
    UserId(user_id): UserId,
) -> Json<CreateWebsiteOutput> {
    if user_id.len() <= 0 {
        return Json(CreateWebsiteOutput {
            website_id: "Not Authenticated".to_string(),
            success: false,
        });
    }
    
    let url = data.url;
    let mut locked_s = s.lock().unwrap();
    let created_website = locked_s.create_website(user_id, url);
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
pub fn get_status(
    Path(website_id): Path<String>,
    Data(s): Data<&Arc<Mutex<Store>>>,
    UserId(user_id): UserId,
) -> Json<GetWebsiteOutput> {
    let mut locked_s = s.lock().unwrap();
    let website_result = locked_s.get_website(website_id, user_id);
    match website_result {
        Ok(w) => Json(GetWebsiteOutput {
            url: w.url,
            success: true,
        }),
        Err(e) => Json(GetWebsiteOutput {
            url: e.to_string(),
            success: true,
        }),
    }
}
