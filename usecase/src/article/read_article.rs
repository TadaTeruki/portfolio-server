use domain::{model::article::GetArticle, repository::article::ArticleRepository};
use std::error::Error;

pub struct ReadArticleUseCase {
    repository: Box<dyn ArticleRepository>,
}

impl ReadArticleUseCase {
    pub fn new(repository_: Box<dyn ArticleRepository>) -> Self {
        Self {
            repository: repository_,
        }
    }

    pub fn execute(&self, id: &str) -> Result<GetArticle, Box<dyn Error>> {
        self.repository.get(id)
    }
}
