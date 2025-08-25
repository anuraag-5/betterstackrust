use crate::config::Config;
use redis::{streams::StreamReadOptions, Connection, RedisError, TypedCommands};

pub struct WebsiteEvent {
    pub url: String,
    pub id: String,
    pub users_id: String
}
pub struct Redis {
    pub conn: Connection,
}

impl Redis {
    pub fn default() -> Result<Self, RedisError> {
        let config = Config::default();
        let establish = redis::Client::open(config.redis_url)?;
        let conn = establish.get_connection()?;

        Ok(Redis { conn })
    }

    fn x_add(&mut self, website: WebsiteEvent) {
        let _ = self.conn.xadd(
            "betteruptime:website",
            "*",
            &[("url", website.url), ("id", website.id)],
        );
    }

    pub fn x_add_bulk(&mut self, websites: Vec<WebsiteEvent>) -> () {

        for website in websites {
            self.x_add(WebsiteEvent {
                url: website.url.clone(),
                id: website.id.clone(),
                users_id: website.users_id.clone()
            });
        }
    }

    pub fn x_read_group(&mut self, consumer_group: String, worker_id: String) -> Result<Option<redis::streams::StreamReadReply>, RedisError> {
        let opts = StreamReadOptions::default().group(consumer_group, worker_id);
        let res = self
            .conn
            .xread_options(&["betteruptime:website"], &[">"], &opts);

        return res;
    }

    fn x_ack(&mut self, consumer_group: String, event_id: String) {
        let _ = self
            .conn
            .xack("betteruptime:website", consumer_group, &[event_id]);
    }
    
    pub fn x_ack_bulk(&mut self, consumer_group: String, event_ids: &[String]) -> () {
        for event_id in event_ids {
            self.x_ack(consumer_group.clone(), event_id.clone());
        }
    }
}
