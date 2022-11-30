use crate::error::ApiError;
use axum::{extract::Path, Extension};
use axum_macros::debug_handler;
use di::DiContainer;
use hyper::StatusCode;
use std::sync::Arc;

#[debug_handler]
pub async fn delete_article(
    Path(id_): Path<String>,
    Extension(container): Extension<Arc<DiContainer>>,
) -> Result<StatusCode, ApiError> {
    let usecase = container.usecase_delete_article();

    let res = usecase.execute(&id_);

    match res.await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(ApiError {
            status: StatusCode::BAD_REQUEST,
            description: String::from("article not found"),
        }),
    }
}
