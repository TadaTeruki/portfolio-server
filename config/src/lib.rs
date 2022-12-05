use std::env;
use std::error::Error;

#[derive(Clone)]
pub struct Config {
    project_id: String,
    private_key: String,
    port: u16,
    mode_release: bool,
}

impl Config {
    pub fn init() -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        let config = Config {
            project_id: env::var("PROJECT_ID")?,
            private_key: env::var("PRIVATE_KEY")?,
            port: env::var("PORT")?.parse::<u16>()?,
            mode_release: env::var("MODE")? == "RELEASE",
        };

        Ok(config)
    }

    pub fn get_project_id(&self) -> &str {
        &self.project_id
    }

    pub fn get_private_key(&self) -> &str {
        &self.private_key
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn is_release_mode(&self) -> bool {
        self.mode_release
    }
}
