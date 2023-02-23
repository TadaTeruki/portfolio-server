use domain::{model::article::PutArticle, repository::article::ArticleRepository};
use std::error::Error;

pub struct UpdateArticleUseCase {
    repository: Box<dyn ArticleRepository + Send + Sync>,
}

impl UpdateArticleUseCase {
    pub fn new(repository_: Box<dyn ArticleRepository + Send + Sync>) -> Self {
        Self {
            repository: repository_,
        }
    }

    pub async fn execute(
        &self,
        id: &str,
        title: String,
        subtitle: String,
        body: String,
        thumbnail: String,
        tags: Vec<String>,
        is_public: bool,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.repository
            .put(
                id,
                PutArticle::new(title, subtitle, body, thumbnail, tags, is_public),
            )
            .await?;
        Ok(())
    }
}
