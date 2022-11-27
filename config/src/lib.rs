use std::env;
use std::error::Error;

pub struct Config {
    project_id: String,
}

impl Config {
    pub fn init() -> Result<Self, Box<dyn Error>> {
        let config = Config {
            project_id: env::var("PROJECT_ID")?,
        };

        Ok(config)
    }

    pub fn get_project_id(&self) -> String {
        self.project_id.clone()
    }
}
