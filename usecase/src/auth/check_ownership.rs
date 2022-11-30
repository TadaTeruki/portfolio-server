use domain::{model::auth::Ownership, repository::auth::AuthRepository};
use std::error::Error;

pub struct CheckOwnershipUseCase {
    repository: Box<dyn AuthRepository + Send + Sync>,
}

impl CheckOwnershipUseCase {
    pub fn new(repository_: Box<dyn AuthRepository + Send + Sync>) -> Self {
        Self {
            repository: repository_,
        }
    }

    pub async fn execute(
        &self,
        name: &str,
        passwd_: &str,
    ) -> Result<Option<Ownership>, Box<dyn Error + Send + Sync + 'static>> {
        let identify = self.repository.query_identification(name).await?;

        match identify {
            Some(passwd) => {
                if passwd.passwd == passwd_ {
                    let res = Ownership::new(name).await?;
                    Ok(Some(res))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }
}
