use crate::config::Config;
use redis::{ AsyncCommands, RedisError, aio::MultiplexedConnection, streams::StreamReadOptions };

pub struct WebsiteEvent {
    pub url: String,
    pub id: String,
    pub users_id: String,
    pub is_snipp_added: bool
}
pub struct Redis {
    pub conn: MultiplexedConnection,
}

impl Redis {
    pub async fn default() -> Result<Self, RedisError> {
        let config = Config::default();
        let client = redis::Client::open(config.redis_url)?;
        let conn = client.get_multiplexed_async_connection().await?;

        Ok(Redis { conn })
    }

    async fn x_add(&mut self, website: &WebsiteEvent) {
        let _: Result<String, RedisError> = self.conn.xadd(
            "betteruptime:website",
            "*",
            &[("url", website.url.clone()), ("id", website.id.clone())],
        ).await;
    }

    pub async fn x_add_bulk(&mut self, websites: &Vec<WebsiteEvent>) -> () {

        for website in websites {
            if website.is_snipp_added {
                self.x_add(website).await;
            }
        }
    }

    pub async fn x_read_group(&mut self, consumer_group: &String, worker_id: &String) -> Result<Option<redis::streams::StreamReadReply>, RedisError> {
        let opts = StreamReadOptions::default().group(consumer_group, worker_id);
        let res = self
            .conn
            .xread_options(&["betteruptime:website"], &[">"], &opts).await;

        return res;
    }

    async fn x_ack(&mut self, consumer_group: &String, event_id: String) {
        let _: Result<String, RedisError>= self
            .conn
            .xack("betteruptime:website", consumer_group, &[event_id]).await;
    }
    
    pub async fn x_ack_bulk(&mut self, consumer_group: &String, event_ids: &[String]) -> () {
        for event_id in event_ids {
            self.x_ack(&consumer_group, event_id.clone()).await;
        }
    }
}
