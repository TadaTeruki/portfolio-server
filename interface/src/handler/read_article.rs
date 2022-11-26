use crate::api_error::ApiError;
use axum::{Extension, Json};
use di::DiContainer;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct ReadArticleRequest {
    id: String,
}

#[derive(Serialize)]
pub struct ReadArticleResponse {
    id: String,
    title: String,
    subtitle: String,
    body: String,
    tags: Vec<String>,
    time_publish: String,
    time_updated: String,
}

pub async fn read_article(
    Json(request): Json<ReadArticleRequest>,
    Extension(container): Extension<Arc<DiContainer>>,
) -> Result<Json<ReadArticleResponse>, ApiError> {
    let usecase = match container.usecase_read_article() {
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
        Ok(article) => Ok(Json(ReadArticleResponse {
            id: request.id,
            title: article.title,
            subtitle: article.subtitle,
            body: article.body,
            tags: article.tags,
            time_publish: article.time_publish,
            time_updated: article.time_updated,
        })),
        Err(_) => Err(ApiError {
            status: StatusCode::BAD_REQUEST,
            description: String::from("article not found"),
        }),
    }
}
