use crate::api_error::ApiError;
use axum::{Extension, Json};
use di::DiContainer;
use hyper::StatusCode;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct DeleteArticleRequest {
    id: String,
}

pub async fn delete_article(
    Json(request): Json<DeleteArticleRequest>,
    Extension(container): Extension<Arc<DiContainer>>,
) -> Result<StatusCode, ApiError> {
    let usecase = match container.usecase_delete_article() {
        Ok(u) => u,
        Err(_) => {
            return Err(ApiError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                description: String::from("internal_server_error"),
            });
        }
    };

    let res = usecase.execute(&request.id);

    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(ApiError {
            status: StatusCode::BAD_REQUEST,
            description: String::from("article not found"),
        }),
    }
}
