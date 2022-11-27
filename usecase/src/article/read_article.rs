use domain::{model::article::GetArticle, repository::article::ArticleRepository};
use std::error::Error;

pub struct ReadArticleUseCase {
    repository: Box<dyn ArticleRepository + Send + Sync>,
}

impl ReadArticleUseCase {
    pub fn new(repository_: Box<dyn ArticleRepository + Send + Sync>) -> Self {
        Self {
            repository: repository_,
        }
    }

    pub async fn execute(
        &self,
        id: &str,
    ) -> Result<Option<GetArticle>, Box<dyn Error + Send + Sync + 'static>> {
        self.repository.get(id).await
    }
}
