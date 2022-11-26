use crate::api_error::ApiError;
use axum::{Extension, Json};
use di::DiContainer;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct PostArticleRequest {
    title: String,
    subtitle: String,
    body: String,
    tags: Vec<String>,
}

#[derive(Serialize)]
pub struct PostArticleResponse {
    id: String,
}

pub async fn post_article(
    Json(request): Json<PostArticleRequest>,
    Extension(container): Extension<Arc<DiContainer>>,
) -> Result<Json<PostArticleResponse>, ApiError> {
    let usecase = match container.usecase_post_article() {
        Ok(u) => u,
        Err(_) => {
            return Err(ApiError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                description: String::from("internal_server_error"),
            });
        }
    };

    let res = usecase.execute(request.title, request.subtitle, request.body, request.tags);

    match res {
        Ok(id_) => Ok(Json(PostArticleResponse { id: id_ })),
        Err(_) => Err(ApiError {
            status: StatusCode::BAD_REQUEST,
            description: String::from("bad request"),
        }),
    }
}
