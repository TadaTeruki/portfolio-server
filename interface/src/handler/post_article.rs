use crate::api_error::ApiError;
use axum::{http::StatusCode, Extension, Json};
use axum_macros::debug_handler;
use di::DiContainer;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct PostArticleRequest {
    title: String,
    subtitle: String,
    body: String,
    tags: Vec<String>,
    is_public: bool,
}

#[derive(Serialize)]
pub struct PostArticleResponse {
    id: String,
}

#[debug_handler]
pub async fn post_article(
    Json(payload): Json<PostArticleRequest>,
    Extension(container): Extension<Arc<DiContainer>>,
) -> Result<Json<PostArticleResponse>, ApiError> {
    let usecase = container.usecase_post_article();

    let res = usecase.execute(
        payload.title,
        payload.subtitle,
        payload.body,
        payload.tags,
        payload.is_public,
    );

    match res.await {
        Ok(id_) => Ok(Json(PostArticleResponse { id: id_ })),
        Err(_) => Err(ApiError {
            status: StatusCode::BAD_REQUEST,
            description: String::from("bad request"),
        }),
    }
}
