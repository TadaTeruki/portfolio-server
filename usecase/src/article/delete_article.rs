use domain::repository::article::ArticleRepository;
use std::error::Error;

pub struct DeleteArticleUseCase {
    repository: Box<dyn ArticleRepository + Send + Sync>,
}

impl DeleteArticleUseCase {
    pub fn new(repository_: Box<dyn ArticleRepository + Send + Sync>) -> Self {
        Self {
            repository: repository_,
        }
    }

    pub async fn execute(&self, id: &str) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.repository.delete(id).await?;
        Ok(())
    }
}
