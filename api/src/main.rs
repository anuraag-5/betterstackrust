use crate::{request_input::{CreateUserInput, CreateWebsiteInput}, request_output::{CreateUserOutput, CreateWebsiteOutput}};
use poem::{
    get, handler,
    listener::TcpListener,
    post,
    web::{Json, Path},
    Route, Server,
};
use dotenvy::dotenv;
use store::{store::Store};

pub mod request_input;
pub mod request_output;

#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let url = data.url;
    let s = Store::default().unwrap();
    let id = s.get_website();
    Json(CreateWebsiteOutput {
        id: format!("Url:{url}, ID: {id}"),
    })
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
        Ok(user_id) => {
            Json(CreateUserOutput { jwt: user_id, success: true })
        }
        Err(e) => {
            Json(CreateUserOutput { jwt: e.to_string(), success: false })
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    let app = Route::new()
        .at("/website/:website_id", get(get_status))
        .at("/website", post(create_website));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
