use domain::{model::article::PostArticle, repository::article::ArticleRepository};
use std::error::Error;

pub struct PostArticleUseCase {
    repository: Box<dyn ArticleRepository>,
}

impl PostArticleUseCase {
    pub fn new(repository_: Box<dyn ArticleRepository>) -> Self {
        Self {
            repository: repository_,
        }
    }

    pub fn execute(
        &self,
        title: String,
        subtitle: String,
        body: String,
        tags: Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        self.repository
            .insert(PostArticle::new(title, subtitle, body, tags))
    }
}
