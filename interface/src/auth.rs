use axum::http::HeaderMap;
use di::DiContainer;
use std::sync::Arc;

pub async fn has_authorization(headers: HeaderMap, container: &Arc<DiContainer>) -> Option<String> {
    if !headers.contains_key("Authorization") || headers["Authorization"].len() < 7 {
        return None;
    }

    let header_token = match headers["Authorization"].to_str() {
        Ok(token) => token.to_string().split_off(7),
        Err(_) => {
            return None;
        }
    };

    let usecase = container.usecase_check_ownership();

    match usecase
        .execute(&header_token, container.config.get_private_key())
        .await
    {
        Ok(claims) => Some(claims.owner_id),
        Err(_) => None,
    }
}
