use crate::auth::has_authorization;
use crate::error::ApiError;
use axum::{http::HeaderMap, http::StatusCode, Extension, Json};
use axum_macros::debug_handler;
use chrono::{DateTime, Utc};
use di::DiContainer;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct ListArticle {
    id: String,
    title: String,
    subtitle: String,
    tags: Vec<String>,
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
    headers: HeaderMap,
    Extension(container): Extension<Arc<DiContainer>>,
) -> Result<Json<ListArticleResponse>, ApiError> {
    let only_public = has_authorization(headers, &container).await.is_none();

    let usecase = container.usecase_list_article();

    let res = usecase.execute(only_public);

    match res.await {
        Ok(raws_) => {
            let mut articles_: Vec<ListArticle> = Vec::new();
            for a in raws_.iter() {
                articles_.push(ListArticle {
                    id: a.id.clone(),
                    title: a.title.clone(),
                    subtitle: a.subtitle.clone(),
                    tags: a.tags.clone(),
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
