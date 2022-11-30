use async_trait::async_trait;
use domain::{model::auth::QueryIdentification, repository::auth::AuthRepository};
use firestore::FirestoreDb;
use std::error::Error;

pub struct AuthDBRepository {
    db: FirestoreDb,
}

impl AuthDBRepository {
    pub fn new(db_: FirestoreDb) -> Self {
        Self { db: db_ }
    }
}

#[async_trait]
impl AuthRepository for AuthDBRepository {
    async fn query_identification(
        &self,
        name: &str,
    ) -> Result<Option<QueryIdentification>, Box<dyn Error + Send + Sync + 'static>> {
        let ownership = self
            .db
            .fluent()
            .select()
            .by_id_in("owners")
            .obj()
            .one(&name)
            .await?;

        Ok(ownership)
    }
}
