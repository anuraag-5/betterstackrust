use crate::config::Config;
use diesel::prelude::*;
use diesel_async:: {AsyncPgConnection, AsyncConnection };

pub struct Store {
    pub conn: AsyncPgConnection
}

impl Store {
    pub async fn default() -> Result<Self, ConnectionError> {
        let config = Config::default();
        let connection = AsyncPgConnection::establish(&config.db_url).await?;
        Ok(Self { conn: connection })
    }
}
