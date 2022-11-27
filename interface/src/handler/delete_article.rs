use crate::api_error::ApiError;
use axum::{Extension, Json};
use axum_macros::debug_handler;
use di::DiContainer;
use hyper::StatusCode;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct DeleteArticleRequest {
    id: String,
}

#[debug_handler]
pub async fn delete_article(
    Json(payload): Json<DeleteArticleRequest>,
    Extension(container): Extension<Arc<DiContainer>>,
) -> Result<StatusCode, ApiError> {
    let usecase = container.usecase_delete_article();

    let res = usecase.execute(&payload.id);

    match res.await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(ApiError {
            status: StatusCode::BAD_REQUEST,
            description: String::from("article not found"),
        }),
    }
}
