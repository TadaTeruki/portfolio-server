use crate::api_error::ApiError;
use axum::{Extension, Json};
use di::DiContainer;
use hyper::StatusCode;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct UpdateArticleRequest {
    id: String,
    title: Option<String>,
    subtitle: Option<String>,
    body: Option<String>,
    tags: Option<Vec<String>>,
    is_public: Option<bool>,
}

pub async fn update_article(
    Json(request): Json<UpdateArticleRequest>,
    Extension(container): Extension<Arc<DiContainer>>,
) -> Result<StatusCode, ApiError> {
    let usecase = match container.usecase_update_article() {
        Ok(u) => u,
        Err(_) => {
            return Err(ApiError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                description: String::from("internal_server_error"),
            });
        }
    };

    let res = usecase.execute(
        request.id,
        request.title,
        request.subtitle,
        request.body,
        request.tags,
        request.is_public,
    );

    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(ApiError {
            status: StatusCode::BAD_REQUEST,
            description: String::from("bad request"),
        }),
    }
}
