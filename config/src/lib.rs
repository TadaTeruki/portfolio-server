use std::env;
use std::error::Error;

const FIRESTORE_CREDENTIALS_SOURCE_KEY: &str = "FIRESTORE_CREDENTIALS_SOURCE";

pub struct Config {
    credentials_src: String,
}

impl Config {
    pub fn init() -> Result<Box<Self>, Box<dyn Error>> {
        let config = Config {
            credentials_src: env::var(FIRESTORE_CREDENTIALS_SOURCE_KEY)?,
        };

        Ok(Box::new(config))
    }

    pub fn get_credentials_src(&self) -> &str {
        &self.credentials_src
    }
}
