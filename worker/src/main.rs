use diesel::prelude::*;
use dotenvy::dotenv;
use redisstreams::redis::Redis;
use reqwest::StatusCode;
use std::{env, io::Error, time::Instant};
use store::schema::website_tick;
use store::{models::website::WebsiteTick, store::Store};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {

    dotenv().ok();
    
    let region = env::var("REGION").map_err(|e| Error::new(std::io::ErrorKind::InvalidInput, e))?;
    let worker_id =
        env::var("WORKER_ID").map_err(|e| Error::new(std::io::ErrorKind::InvalidInput, e))?;

    if region.len() == 0 || worker_id.len() == 0 {
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid env inputs",
        ));
    }

    let mut r = Redis::default().map_err(|e| {
        Error::new(
            std::io::ErrorKind::ConnectionRefused,
            format!("Redis connection error. {}", e),
        )
    })?;

    let mut str =
        Store::default().map_err(|e| Error::new(std::io::ErrorKind::ConnectionRefused, e))?;

    let messages = r.x_read_group(region, worker_id).map_err(|e| {
        Error::new(
            std::io::ErrorKind::InvalidData,
            format!("XREADGROUP Error {}", e),
        )
    })?;

    match messages {
        Some(s) => {
            let streams = s.keys;
            for stream in streams {
                let stream_ids = stream.ids;

                for stream_id in stream_ids {
                    let map = stream_id.map;
                    let url_value = map.get("url").unwrap();
                    let website_id_value = map.get("id").unwrap();

                    let url = redis::from_redis_value::<String>(url_value).unwrap();
                    let website_id = redis::from_redis_value::<String>(website_id_value).unwrap();

                    fetch_website(&mut str, url, website_id).await;
                }
            }
        }
        None => {}
    }

    Ok(())
}

async fn fetch_website(s: &mut Store, url: String, website_id: String) {

    let start_time = Instant::now();

    let res = reqwest::get(format!("https://{}", url)).await;

    let total_time = start_time.elapsed().as_millis() as i32;

    match res {

        Ok(rps) => {

            if rps.status() == StatusCode::OK {

                let website_tick = WebsiteTick {
                    id: Uuid::new_v4().to_string(),
                    response_time_ms: total_time,
                    status: "Up".to_owned(),
                    region_id: "1".to_owned(),
                    website_id,
                };

                let _ = diesel::insert_into(website_tick::table)
                    .values(website_tick)
                    .returning(WebsiteTick::as_returning())
                    .get_result(&mut s.conn);
            } else {

                let website_tick = WebsiteTick {
                    id: Uuid::new_v4().to_string(),
                    response_time_ms: total_time,
                    status: "Down".to_owned(),
                    region_id: "1".to_owned(),
                    website_id,
                };

                let _ = diesel::insert_into(website_tick::table)
                    .values(website_tick)
                    .returning(WebsiteTick::as_returning())
                    .get_result(&mut s.conn);
            }
        }

        Err(_) => {
            let website_tick = WebsiteTick {
                id: Uuid::new_v4().to_string(),
                response_time_ms: total_time,
                status: "Unknown".to_owned(),
                region_id: "1".to_owned(),
                website_id
            };

            let _ = diesel::insert_into(website_tick::table)
            .values(website_tick)
            .returning(WebsiteTick::as_returning())
            .get_result(&mut s.conn);
        }
    }
}
