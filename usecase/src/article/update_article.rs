use domain::{model::article::PutArticle, repository::article::ArticleRepository};
use std::error::Error;

pub struct UpdateArticleUseCase {
    repository: Box<dyn ArticleRepository>,
}

impl UpdateArticleUseCase {
    pub fn new(repository_: Box<dyn ArticleRepository>) -> Self {
        Self {
            repository: repository_,
        }
    }

    pub fn execute(
        &self,
        id: String,
        title: Option<String>,
        subtitle: Option<String>,
        body: Option<String>,
        tags: Option<Vec<String>>,
        is_public: Option<bool>,
    ) -> Result<(), Box<dyn Error>> {
        self.repository
            .put(&id, PutArticle::new(title, subtitle, body, tags, is_public))
    }
}
