use std::sync::{Arc, Mutex};

use crate::{
    request_input::{CreateUserInput, CreateWebsiteInput},
    request_output::{CreateUserOutput, CreateWebsiteOutput, GetWebsiteOutput, SigninUserOutput},
};
use dotenvy::dotenv;
use poem::{
    get, handler,
    listener::TcpListener,
    post,
    web::{Data, Json, Path},
    EndpointExt, Route, Server,
};
use store::store::Store;

pub mod request_input;
pub mod request_output;

#[handler]
fn create_website(
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
fn get_status(
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

#[handler]
fn create_user(
    Json(data): Json<CreateUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<CreateUserOutput> {
    let username = data.username;
    let user_password = data.password;

    let mut locked_s = s.lock().unwrap();
    let result = locked_s.sign_up(username, user_password);

    match result {
        Ok(user_id) => Json(CreateUserOutput {
            user_id: user_id,
            success: true,
        }),
        Err(e) => Json(CreateUserOutput {
            user_id: e.to_string(),
            success: false,
        }),
    }
}

#[handler]
fn sign_in_user(
    Json(data): Json<CreateUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<SigninUserOutput> {
    let username = data.username;
    let user_password = data.password;

    let mut locked_s = s.lock().unwrap();
    let result = locked_s.sign_in(username, user_password);

    match result {
        Ok(exists) => {
            if exists {
                return Json(SigninUserOutput {
                    jwt: String::from("Temporary JWT"),
                    success: true,
                });
            } else {
                return Json(SigninUserOutput {
                    jwt: String::from("Temporary JWT"),
                    success: false,
                });
            }
        }
        Err(e) => Json(SigninUserOutput {
            jwt: e.to_string(),
            success: false,
        }),
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    let s = Arc::new(Mutex::new(Store::default().unwrap()));
    let app = Route::new()
        .at("/website/:website_id", get(get_status))
        .at("/website", post(create_website))
        .at("/user/signup", post(create_user))
        .at("/user/signin", get(sign_in_user))
        .data(s);
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
