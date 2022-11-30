use crate::error::ApiError;
use axum::{Extension, Json};
use axum_macros::debug_handler;
use di::DiContainer;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct LoginAsOwnerRequest {
    name: String,
    passwd: String,
}

#[derive(Serialize)]
pub struct LoginAsOwnerResponse {
    token: String,
}

#[debug_handler]
pub async fn login_as_owner(
    Json(payload): Json<LoginAsOwnerRequest>,
    Extension(container): Extension<Arc<DiContainer>>,
) -> Result<Json<LoginAsOwnerResponse>, ApiError> {
    let usecase = container.usecase_check_ownership();

    let res = usecase.execute(&payload.name, &payload.passwd);

    match res.await {
        Ok(opt) => match opt {
            Some(ownership) => Ok(Json(LoginAsOwnerResponse {
                token: ownership.token,
            })),
            None => Err(ApiError {
                status: StatusCode::BAD_REQUEST,
                description: String::from("owner's name or password is incorrect"),
            }),
        },
        Err(_) => Err(ApiError {
            status: StatusCode::BAD_REQUEST,
            description: String::from("invalid request"),
        }),
    }
}
