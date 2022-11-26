use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

fn get_utctime() -> String {
    Utc::now().format("%Y-%m-%d-%H-%M-%S").to_string()
}

#[derive(Serialize)]
pub struct PostArticle {
    title: String,
    subtitle: String,
    body: String,
    tags: Vec<String>,
    time_publish: String,
    time_updated: String,
}

impl PostArticle {
    pub fn new(title_: String, subtitle_: String, body_: String, tags_: Vec<String>) -> Self {
        Self {
            title: title_,
            subtitle: subtitle_,
            body: body_,
            tags: tags_,
            time_publish: get_utctime(),
            time_updated: get_utctime(),
        }
    }

    pub fn allocate_new_uuid(&self) -> String {
        Uuid::new_v4().to_hyphenated().to_string()
    }
}

#[derive(Deserialize)]
pub struct GetArticle {
    pub title: String,
    pub subtitle: String,
    pub body: String,
    pub tags: Vec<String>,
    pub time_publish: String,
    pub time_updated: String,
}
