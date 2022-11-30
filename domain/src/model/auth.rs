use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct JwtClaims {
    account_id: String,
    iat: i64,
    exp: i64,
}

#[derive(Deserialize)]
pub struct QueryIdentification {
    pub passwd: String,
}

pub struct Ownership {
    pub token: String,
}

impl Ownership {
    pub async fn new(name: &str) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        let mut header = Header::new(Algorithm::HS256);
        header.typ = Some("JWT".to_string());
        let now = Utc::now();
        let claims = JwtClaims {
            account_id: name.to_string(),
            iat: now.timestamp(),
            exp: (now + Duration::hours(8)).timestamp(),
        };

        match encode(
            &header,
            &claims,
            &EncodingKey::from_secret("secret!!".as_ref()),
        ) {
            Ok(token_) => Ok(Ownership { token: token_ }),
            Err(err) => Err(Box::new(err)),
        }
    }
}
