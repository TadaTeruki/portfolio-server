use crate::api_error::ApiError;
use axum::{http::StatusCode, Extension, Json};
use axum_macros::debug_handler;
use chrono::{DateTime, Utc};
use di::DiContainer;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct ListArticleRequest {
    only_public: bool,
}

#[derive(Serialize)]
pub struct ListArticle {
    title: String,
    subtitle: String,
    is_public: bool,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct ListArticleResponse {
    articles: Vec<ListArticle>,
}

#[debug_handler]
pub async fn list_article(
    Json(payload): Json<ListArticleRequest>,
    Extension(container): Extension<Arc<DiContainer>>,
) -> Result<Json<ListArticleResponse>, ApiError> {
    let usecase = container.usecase_list_article();

    let res = usecase.execute(payload.only_public);

    match res.await {
        Ok(raws_) => {
            let mut articles_: Vec<ListArticle> = Vec::new();
            for a in raws_.iter() {
                articles_.push(ListArticle {
                    title: a.title.clone(),
                    subtitle: a.subtitle.clone(),
                    is_public: a.is_public,
                    created_at: a.created_at,
                    updated_at: a.updated_at,
                })
            }
            Ok(Json(ListArticleResponse {
                articles: articles_,
            }))
        }
        Err(_) => Err(ApiError {
            status: StatusCode::BAD_REQUEST,
            description: String::from("bad request"),
        }),
    }
}
