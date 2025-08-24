use std::io::Error;
use dotenvy::dotenv;

use redisstreams::redis::{Redis, WebsiteEvent};
use store::store::Store;

fn main() -> Result<(), Error> {
    dotenv().ok();
    let mut r = Redis::default().map_err(|error| Error::new(std::io::ErrorKind::NotConnected, error))?;
    let mut s = Store::default().map_err(|e| Error::new(std::io::ErrorKind::NotConnected, e))?;

    let websites =  s.get_users_website().map_err(|e| Error::new(std::io::ErrorKind::NotConnected, e))?;

    let website_events: Vec<WebsiteEvent> = websites.into_iter().map(|w| {
        let ( url, id ) = w;
        WebsiteEvent {
            url,
            id
        }
    }).collect();
    
    r.x_add_bulk(website_events);
    

    Ok(())
}
