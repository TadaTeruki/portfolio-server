use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct PostArticle {
    title: String,
    subtitle: String,
    body: String,
    tags: Vec<String>,
    is_public: bool,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
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
            created_at: Utc::now(),
            updated_at: Some(Utc::now()),
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
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize)]
pub struct PutArticle {
    pub title: String,
    pub subtitle: String,
    pub body: String,
    pub tags: Vec<String>,
    pub is_public: bool,
    pub updated_at: Option<DateTime<Utc>>,
}

impl PutArticle {
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
            updated_at: Some(Utc::now()),
        }
    }
}
