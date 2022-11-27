use crate::model::article::{GetArticle, PostArticle, PutArticle};
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait ArticleRepository {
    async fn delete(&self, id: &str) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;
    async fn get(
        &self,
        id: &str,
    ) -> Result<Option<GetArticle>, Box<dyn Error + Send + Sync + 'static>>;
    //fn list(&self) -> Result<Vec<Article>, Box<dyn Error>>;
    async fn insert(
        &self,
        article: PostArticle,
    ) -> Result<String, Box<dyn Error + Send + Sync + 'static>>;
    async fn put(
        &self,
        id: &str,
        article: PutArticle,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;
}
