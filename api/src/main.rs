use dotenvy::dotenv;
use poem::http::Method;
use poem::{get, listener::TcpListener, post, EndpointExt, Route, Server};

use poem::middleware::{Cors, CookieJarManager};
use std::{io::Error, sync::{Arc, Mutex}};
use store::store::Store;
use crate::route::app::{get_user, total_views};
use crate::route::user::logout_user;
use crate::route::{
    app::{snippet, track}, user::{create_user, sign_in_user}, website::{create_website, get_status}
};

pub mod request_input;
pub mod request_output;
pub mod route;
pub mod auth_middleware;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    dotenv().ok();

    let s = Arc::new(Mutex::new(Store::default().map_err(|e| Error::new(std::io::ErrorKind::NotConnected, e))?));

    let cors = Cors::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_credentials(true);

    let app = Route::new()
        .at("/api/website/:website_id", get(get_status))
        .at("/api/website", post(create_website))
        .at("/api/user/signup", post(create_user))
        .at("/api/user/signin", post(sign_in_user))
        .at("/api/snippet", get(snippet))
        .at("/api/track", post(track))
        .at("/api/totalviews/:w_id", get(total_views))
        .at("/api/get_user", get(get_user))
        .at("/api/user/logout", post(logout_user))
        .data(s)
        .with(cors)
        .with(CookieJarManager::new());

    Server::new(TcpListener::bind("0.0.0.0:3001"))
        .run(app)
        .await
}
