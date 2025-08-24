use std::env;

pub struct Config {
    pub redis_url: String
}

impl Default for Config {
    fn default() -> Self {
        let redis_url =
            env::var("REDIS_URL").unwrap_or_else(|_| panic!("Please provide redis url"));

        Self {
            redis_url,
        }
    }
}