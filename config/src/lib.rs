use std::env;
use std::error::Error;

pub struct Config {
    project_id: String,
    private_key: String,
}

impl Config {
    pub fn init() -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        let config = Config {
            project_id: env::var("PROJECT_ID")?,
            private_key: env::var("PRIVATE_KEY")?,
        };

        Ok(config)
    }

    pub fn get_project_id(&self) -> &str {
        &self.project_id
    }

    pub fn get_private_key(&self) -> &str {
        &self.private_key
    }
}
