use std::time::Duration;
use tokio::time::sleep;
use dotenvy::dotenv;
use redisstreams::redis::{Redis, WebsiteEvent};
use store::store::Store;

async fn main_loop() -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();
    let mut r = Redis::default().await?;
    let mut s = Store::new().await;

    loop {
        let websites = s.get_all_websites().await?;

        let website_events: Vec<WebsiteEvent> = websites.into_iter().map(|w| {
            let (url, id, users_id, is_snipp_added) = w;
            WebsiteEvent { url, id, users_id, is_snipp_added }
        }).collect();

        r.x_add_bulk(&website_events).await;
        sleep(Duration::from_secs(10)).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    main_loop().await
}