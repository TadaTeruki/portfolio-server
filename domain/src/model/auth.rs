use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct JwtClaims {
    pub owner_id: String,
    iat: i64,
    exp: i64,
}

#[derive(Deserialize)]
pub struct QueryIdentification {
    pub passwd: String,
    pub owner_id: String,
}

pub struct Ownership {
    pub token: String,
}

impl Ownership {
    pub async fn generate(
        id: &str,
        private_key: &str,
    ) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        let mut header = Header::new(Algorithm::HS256);
        header.typ = Some("JWT".to_string());
        let now = Utc::now();
        let claims = JwtClaims {
            owner_id: id.to_string(),
            iat: now.timestamp(),
            exp: (now + Duration::hours(8)).timestamp(),
        };

        match encode(
            &header,
            &claims,
            &EncodingKey::from_secret(private_key.as_ref()),
        ) {
            Ok(token_) => Ok(Ownership { token: token_ }),
            Err(err) => Err(Box::new(err)),
        }
    }

    pub async fn decode(
        &self,
        private_key: &str,
    ) -> Result<JwtClaims, Box<dyn Error + Send + Sync + 'static>> {
        match decode::<JwtClaims>(
            &self.token,
            &DecodingKey::from_secret(private_key.as_ref()),
            &Validation::default(),
        ) {
            Ok(tokendata) => Ok(tokendata.claims),
            Err(err) => Err(Box::new(err)),
        }
    }
}

/*
use jsonwebtoken::{TokenData, DecodingKey, Validation, decode};
fn decode_jwt(jwt: &str, secret: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = std::env::var(secret).expect("secret is not set");
    decode::<Claims>(
        jwt, &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default())
}
*/
