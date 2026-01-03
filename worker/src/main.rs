use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use dotenvy::dotenv;
use redis::RedisError;
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

    let _ = ensure_group(&mut r, "betteruptime:website", &region).await;

    let mut str = Store::new().await;

    loop {
        let cloned_region = region.clone();
        let cloned_worker_id = worker_id.clone();

        let messages = match r.x_read_group(&cloned_region, &cloned_worker_id).await {
            Ok(m) => m,
            Err(e) if e.to_string().contains("NOGROUP") => {
                let _ = ensure_group(&mut r, "betteruptime:website", &cloned_region).await;
                continue;
            }
            Err(e) => return Err(Error::new(std::io::ErrorKind::Other, e)),
        };

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
    let mut conn = s.pool.get().await.map_err(|e| {
        println!("{}", e.to_string());
        return Error::new(std::io::ErrorKind::ConnectionRefused, e);
    })?;
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
                    .get_result(&mut conn)
                    .await;

                match val {
                    Ok(w) => {
                        println!("{}", w.response_time_ms);
                        return Ok(());
                    }
                    Err(_) => {
                        return Ok(());
                    }
                }
            } else {
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
                    .get_result(&mut conn)
                    .await;

                return Ok(());
            }
        }

        Err(_) => {
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
                .get_result(&mut conn)
                .await;

            return Ok(());
        }
    }
}

async fn ensure_group(r: &mut Redis, stream: &str, group: &str) -> Result<(), RedisError> {
    let res: Result<(), RedisError> = redis::cmd("XGROUP")
        .arg("CREATE")
        .arg(stream)
        .arg(group)
        .arg("$")
        .arg("MKSTREAM")
        .query_async(&mut r.conn)
        .await;

    // Ignore BUSYGROUP error
    Ok(())
}
