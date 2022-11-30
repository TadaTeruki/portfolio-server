pub struct CheckOwnershipUseCase {}
use domain::model::auth::{JwtClaims, Ownership};
use std::error::Error;

impl CheckOwnershipUseCase {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn execute(
        &self,
        header_token: &str,
        private_key: &str,
    ) -> Result<JwtClaims, Box<dyn Error + Send + Sync + 'static>> {
        let ownership = Ownership {
            token: header_token.to_string(),
        };

        let res = ownership.decode(private_key).await?;
        Ok(res)
    }
}

impl Default for CheckOwnershipUseCase {
    fn default() -> Self {
        Self::new()
    }
}
