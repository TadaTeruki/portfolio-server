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
    is_public: bool,
    time_publish: String,
    time_updated: String,
}

impl PostArticle {
    pub fn new(
        title_: String,
        subtitle_: String,
        body_: String,
        tags_: Vec<String>,
        is_public_: bool,
    ) -> Self {
        Self {
            title: title_,
            subtitle: subtitle_,
            body: body_,
            tags: tags_,
            is_public: is_public_,
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
    pub is_public: bool,
    pub time_publish: String,
    pub time_updated: String,
}

#[derive(Serialize)]
pub struct PutArticle {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subtitle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_public: Option<bool>,

    time_updated: String,
}

impl PutArticle {
    pub fn new(
        title_: Option<String>,
        subtitle_: Option<String>,
        body_: Option<String>,
        tags_: Option<Vec<String>>,
        is_public_: Option<bool>,
    ) -> Self {
        Self {
            title: title_,
            subtitle: subtitle_,
            body: body_,
            tags: tags_,
            is_public: is_public_,
            time_updated: get_utctime(),
        }
    }
}
