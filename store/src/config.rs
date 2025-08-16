use std::env;

pub struct Config {
    pub db_url: String,
}

impl Default for Config {
    fn default() -> Self {
        let database_url =
            env::var("DATABASE_URL").unwrap_or_else(|_| panic!("Please provide database url"));

        Self {
            db_url: database_url,
        }
    }
}
