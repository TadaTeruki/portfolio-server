use crate::error::ApiError;
use axum::{extract::Path, Extension, Json};
use axum_macros::debug_handler;
use chrono::{DateTime, Utc};
use di::DiContainer;
use hyper::StatusCode;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct ReadArticleResponse {
    id: String,
    title: String,
    subtitle: String,
    body: String,
    tags: Vec<String>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

#[debug_handler]
pub async fn read_article(
    Path(id_): Path<String>,
    Extension(container): Extension<Arc<DiContainer>>,
) -> Result<Json<ReadArticleResponse>, ApiError> {
    let usecase = container.usecase_read_article();

    let res = usecase.execute(&id_);

    match res.await {
        Ok(article_opt) => match article_opt {
            Some(article) => Ok(Json(ReadArticleResponse {
                id: id_,
                title: article.title,
                subtitle: article.subtitle,
                body: article.body,
                tags: article.tags,
                created_at: article.created_at,
                updated_at: article.updated_at,
            })),
            None => Err(ApiError {
                status: StatusCode::NOT_FOUND,
                description: String::from("article not found"),
            }),
        },
        Err(_) => Err(ApiError {
            status: StatusCode::BAD_REQUEST,
            description: String::from("bad request"),
        }),
    }
}
