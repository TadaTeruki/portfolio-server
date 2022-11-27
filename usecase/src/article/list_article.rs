use domain::{model::article::ListArticle, repository::article::ArticleRepository};
use std::error::Error;

pub struct ListArticleUseCase {
    repository: Box<dyn ArticleRepository + Send + Sync>,
}

impl ListArticleUseCase {
    pub fn new(repository_: Box<dyn ArticleRepository + Send + Sync>) -> Self {
        Self {
            repository: repository_,
        }
    }

    pub async fn execute(
        &self,
        only_public: bool,
    ) -> Result<Vec<ListArticle>, Box<dyn Error + Send + Sync + 'static>> {
        match only_public {
            true => self.repository.list_public().await,
            false => self.repository.list().await,
        }
    }
}
