use crate::error::ApiError;
use axum::{extract::Path, Extension, Json};
use axum_macros::debug_handler;
use di::DiContainer;
use hyper::StatusCode;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct UpdateArticleRequest {
    title: String,
    subtitle: String,
    body: String,
    tags: Vec<String>,
    is_public: bool,
}

#[debug_handler]
pub async fn update_article(
    Path(id_): Path<String>,
    Json(payload): Json<UpdateArticleRequest>,
    Extension(container): Extension<Arc<DiContainer>>,
) -> Result<StatusCode, ApiError> {
    let usecase = container.usecase_update_article();

    let res = usecase.execute(
        &id_,
        payload.title,
        payload.subtitle,
        payload.body,
        payload.tags,
        payload.is_public,
    );

    match res.await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(ApiError {
            status: StatusCode::BAD_REQUEST,
            description: String::from("bad request"),
        }),
    }
}
