use std::sync::{Arc, Mutex};

use crate::{
    request_input::CreateWebsiteInput,
    request_output::{CreateWebsiteOutput, GetWebsiteOutput},
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
) -> Json<CreateWebsiteOutput> {
    let url = data.url;
    let mut locked_s = s.lock().unwrap();
    let created_website =
        locked_s.create_website(String::from("44e42f9-28b6-4eaa-94a0-77a5a4051b07"), url);
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
) -> Json<GetWebsiteOutput> {
    let mut locked_s = s.lock().unwrap();
    let website_result = locked_s.get_website(website_id);
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
