use crate::{
    request_input::{CreateUserInput, CreateWebsiteInput},
    request_output::{CreateUserOutput, CreateWebsiteOutput, SigninUserOutput},
};
use dotenvy::dotenv;
use poem::{
    get, handler,
    listener::TcpListener,
    post,
    web::{Json, Path},
    Route, Server,
};
use store::store::Store;

pub mod request_input;
pub mod request_output;

#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let url = data.url;
    let mut s = Store::default().unwrap();
    let created_website =
        s.create_website(String::from("58ed4908-0b4a-4e67-aa15-1ed1d7a1f882"), url);
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
fn get_status(Path(website_id): Path<String>) -> String {
    format!("Active: {}", website_id)
}

#[handler]
fn create_user(Json(data): Json<CreateUserInput>) -> Json<CreateUserOutput> {
    let username = data.username;
    let user_password = data.password;

    let mut s = Store::default().unwrap();
    let result = s.sign_up(username, user_password);

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
fn sign_in_user(Json(data): Json<CreateUserInput>) -> Json<SigninUserOutput>{
    let username = data.username;
    let user_password = data.password;

    let mut s = Store::default().unwrap();
    let result = s.sign_in(username, user_password);

    match result {
        Ok(exists) => {
            if exists {
                return Json(SigninUserOutput { jwt: String::from("Temporary JWT"), success: true })
            } else {
                return Json(SigninUserOutput { jwt: String::from("Temporary JWT"), success: false })
            }
        },
        Err(e) => Json(SigninUserOutput { jwt: e.to_string(), success: false })
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    let app = Route::new()
        .at("/website/:website_id", get(get_status))
        .at("/website", post(create_website))
        .at("/user/signup", post(create_user))
        .at("/user/signin", get(sign_in_user));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
