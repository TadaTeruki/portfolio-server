use domain::{model::article::PostArticle, repository::article::ArticleRepository};
use std::error::Error;

pub struct PostArticleUseCase {
    repository: Box<dyn ArticleRepository + Send + Sync>,
}

impl PostArticleUseCase {
    pub fn new(repository_: Box<dyn ArticleRepository + Send + Sync>) -> Self {
        Self {
            repository: repository_,
        }
    }

    pub async fn execute(
        &self,
        title: String,
        subtitle: String,
        body: String,
        thumbnail: String,
        tags: Vec<String>,
        is_public: bool,
    ) -> Result<String, Box<dyn Error + Send + Sync + 'static>> {
        let res = self
            .repository
            .insert(PostArticle::new(
                title, subtitle, body, thumbnail, tags, is_public,
            ))
            .await?;
        Ok(res)
    }
}
