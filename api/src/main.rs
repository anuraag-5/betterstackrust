use poem::{get, handler, listener::TcpListener, post, web::{Json, Path}, Route, Server};
use store::Store;
use crate::{request_input::CreateWebsiteInput, request_output::CreateWebsiteOutput};

pub mod request_input;
pub mod request_output;

#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let url = data.url;
    let s = Store{};
    let id = s.create_website();
    Json(CreateWebsiteOutput {
        id: format!("Url:{}, ID: {}",url, id)
    })
}

#[handler]
fn get_status(Path(website_id): Path<String>) -> String {
    format!("Active: {}", website_id)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/website/:website_id", get(get_status))
        .at("/website", post(create_website));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
