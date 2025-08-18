use dotenvy::dotenv;
use poem::{get, listener::TcpListener, post, EndpointExt, Route, Server};
use std::sync::{Arc, Mutex};
use store::store::Store;

use crate::route::{
    user::{create_user, sign_in_user},
    website::{create_website, get_status},
};

pub mod request_input;
pub mod request_output;
pub mod route;

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
