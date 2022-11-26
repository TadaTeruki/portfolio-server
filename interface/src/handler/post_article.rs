use axum::{response::IntoResponse, Extension, Json};
use di::DiContainer;
use hyper::StatusCode;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct PostArticleRequest {
    title: String,
    subtitle: String,
    body: String,
    tags: Vec<String>,
}

pub async fn post_article(
    Json(request): Json<PostArticleRequest>,
    Extension(container): Extension<Arc<DiContainer>>,
) -> impl IntoResponse {
    let usecase = match container.usecase_post_article() {
        Ok(u) => u,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "internal_server_error");
        }
    };

    let res = usecase.execute(request.title, request.subtitle, request.body, request.tags);

    match res {
        Ok(_) => (StatusCode::OK, "post"),
        Err(_) => (StatusCode::BAD_REQUEST, "bad request"),
    }
}
