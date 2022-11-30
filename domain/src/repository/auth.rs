use crate::model::auth::QueryIdentification;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait AuthRepository {
    async fn query_identification(
        &self,
        name: &str,
    ) -> Result<Option<QueryIdentification>, Box<dyn Error + Send + Sync + 'static>>;
}
