use crate::config::Config;
use diesel::prelude::*;

pub struct Store {
    pub conn: PgConnection,
}

impl Store {
    pub fn default() -> Result<Self, ConnectionError> {
        let config = Config::default();
        let connection = PgConnection::establish(&config.db_url)?;
        Ok(Self { conn: connection })
    }

    pub fn get_all_websites(&mut self) -> Result<Vec<(String, String, String)>, diesel::result::Error> {
        use crate::schema::website::dsl::*;
    
        let websites = website
            .select((url, id, user_id))
            .load::<(String, String, String)>(&mut self.conn)?;
    
        Ok(websites)
    }
}
