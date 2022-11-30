use domain::{model::auth::Ownership, repository::auth::AuthRepository};
use std::error::Error;

pub struct LoginAsOwnerUseCase {
    repository: Box<dyn AuthRepository + Send + Sync + 'static>,
}

impl LoginAsOwnerUseCase {
    pub fn new(repository_: Box<dyn AuthRepository + Send + Sync + 'static>) -> Self {
        Self {
            repository: repository_,
        }
    }

    pub async fn execute(
        &self,
        name: &str,
        passwd_: &str,
        private_key: &str,
    ) -> Result<Option<Ownership>, Box<dyn Error + Send + Sync + 'static>> {
        let identify = self.repository.query_identification(name).await?;

        match identify {
            Some(owner_data) => {
                if owner_data.passwd == passwd_ {
                    let res = Ownership::generate(&owner_data.owner_id, private_key).await?;
                    Ok(Some(res))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }
}
