use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ListArticle {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub tags: Vec<String>,
    pub thumbnail: String,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize)]
pub struct PostArticle {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub body: String,
    pub thumbnail: String,
    pub tags: Vec<String>,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct GetArticle {
    pub title: String,
    pub subtitle: String,
    pub body: String,
    pub thumbnail: String,
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
    pub thumbnail: String,
    pub tags: Vec<String>,
    pub is_public: bool,
    pub updated_at: Option<DateTime<Utc>>,
}

impl PostArticle {
    pub fn new(
        title_: String,
        subtitle_: String,
        body_: String,
        thumbnail_: String,
        tags_: Vec<String>,
        is_public_: bool,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            title: title_,
            subtitle: subtitle_,
            body: body_,
            thumbnail: thumbnail_,
            tags: tags_,
            is_public: is_public_,
            created_at: Utc::now(),
            updated_at: Some(Utc::now()),
        }
    }
}

impl PutArticle {
    pub fn new(
        title_: String,
        subtitle_: String,
        body_: String,
        thumbnail_: String,
        tags_: Vec<String>,
        is_public_: bool,
    ) -> Self {
        Self {
            title: title_,
            subtitle: subtitle_,
            body: body_,
            thumbnail: thumbnail_,
            tags: tags_,
            is_public: is_public_,
            updated_at: Some(Utc::now()),
        }
    }
}
