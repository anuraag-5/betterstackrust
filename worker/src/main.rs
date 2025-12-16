use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use dotenvy::dotenv;
use redisstreams::redis::Redis;
use reqwest::StatusCode;
use std::time::Duration;
use std::{env, io::Error, time::Instant};
use store::schema::website_tick;
use store::{models::website::WebsiteTick, store::Store};
use tokio::time::sleep;
use uuid::Uuid;

async fn main_loop() -> Result<(), Error> {
    let region = env::var("REGION").map_err(|e| Error::new(std::io::ErrorKind::InvalidInput, e))?;

    let worker_id =
        env::var("WORKER_ID").map_err(|e| Error::new(std::io::ErrorKind::InvalidInput, e))?;

    if region.len() == 0 || worker_id.len() == 0 {
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid env inputs",
        ));
    }

    let mut r = Redis::default().await.map_err(|e| {
        Error::new(
            std::io::ErrorKind::ConnectionRefused,
            format!("Redis connection error. {}", e),
        )
    })?;

    let mut str =
        Store::default().await.map_err(|e| Error::new(std::io::ErrorKind::ConnectionRefused, e))?;

    loop {
        let cloned_region = region.clone();
        let cloned_worker_id = worker_id.clone();

        let messages = r
            .x_read_group(&cloned_region, &cloned_worker_id)
            .await.map_err(|e| {
                Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("XREADGROUP Error {}", e),
                )
            })?;

        match messages {
            Some(s) => {
                let streams = s.keys;
                for stream in streams {
                    let stream_name = stream.key;

                    println!("{}", stream_name);
                
                    for stream_id in stream.ids {
                        let message_id = stream_id.id;
                        let map = stream_id.map;
                
                        let url_value = map.get("url").unwrap();
                        let url = redis::from_redis_value::<String>(url_value).unwrap();
                        println!("{}", url);
                        let _ = fetch_website(&mut str, url).await;
                
                        // ✅ ACK MESSAGE
                        r.x_ack_bulk(&cloned_region, &[message_id]).await;
                    }
                }
            }
            None => {}
        }

        sleep(Duration::from_secs(10)).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    main_loop().await?;
    Ok(())
}

async fn fetch_website(s: &mut Store, url: String) -> Result<(), Error> {
    let start_time = Instant::now();
    let region = env::var("REGION").map_err(|e| Error::new(std::io::ErrorKind::InvalidInput, e))?;
    let res = reqwest::get(format!("https://{}", url)).await;

    let total_time = start_time.elapsed().as_millis() as i32;

    match res {
        Ok(rps) => {
            if rps.status() == StatusCode::OK {
                println!("Ok");
                let website_tick = WebsiteTick {
                    id: Uuid::new_v4().to_string(),
                    response_time_ms: total_time,
                    status: "Up".to_owned(),
                    region: region,
                    website_url: url,
                };

                let val = diesel::insert_into(website_tick::table)
                    .values(&website_tick)
                    .returning(WebsiteTick::as_returning())
                    .get_result(&mut s.conn).await;

                match val {
                    Ok(w) => {
                        println!("{}", w.response_time_ms);
                        return Ok(());
                    },
                    Err(e) => {
                        println!("DB insert error: {:?}", e);
                        return Ok(());
                    }
                }
            } else {
                println!("Not Ok");
                let website_tick = WebsiteTick {
                    id: Uuid::new_v4().to_string(),
                    response_time_ms: total_time,
                    status: "Down".to_owned(),
                    region: region,
                    website_url: url,
                };

                let _ = diesel::insert_into(website_tick::table)
                    .values(website_tick)
                    .returning(WebsiteTick::as_returning())
                    .get_result(&mut s.conn).await;

                return Ok(());
            }
        }

        Err(e) => {
            println!("Unknown: {}", e);
            let website_tick = WebsiteTick {
                id: Uuid::new_v4().to_string(),
                response_time_ms: total_time,
                status: "Unknown".to_owned(),
                region: region,
                website_url: url,
            };

            let _ = diesel::insert_into(website_tick::table)
                .values(website_tick)
                .returning(WebsiteTick::as_returning())
                .get_result(&mut s.conn).await;

            return Ok(());
        }
    }
}
