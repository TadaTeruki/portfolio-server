use crate::auth::has_authorization;
use crate::error::ApiError;
use axum::{http::HeaderMap, Extension, Json};
use axum_macros::debug_handler;
use di::DiContainer;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct CheclAuthorizationResponse {
    owner_id: Option<String>,
}

#[debug_handler]
pub async fn check_authorization(
    headers: HeaderMap,
    Extension(container): Extension<Arc<DiContainer>>,
) -> Result<Json<CheclAuthorizationResponse>, ApiError> {
    let authorization = has_authorization(headers, &container).await;

    Ok(Json(CheclAuthorizationResponse {
        owner_id: authorization,
    }))
}
