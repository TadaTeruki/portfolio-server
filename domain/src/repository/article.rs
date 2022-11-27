use crate::model::article::{GetArticle, ListArticle, PostArticle, PutArticle};
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait ArticleRepository {
    async fn delete(&self, id: &str) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    async fn get(
        &self,
        id: &str,
    ) -> Result<Option<GetArticle>, Box<dyn Error + Send + Sync + 'static>>;

    async fn insert(
        &self,
        article: PostArticle,
    ) -> Result<String, Box<dyn Error + Send + Sync + 'static>>;

    async fn put(
        &self,
        id: &str,
        article: PutArticle,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    async fn list(&self) -> Result<Vec<ListArticle>, Box<dyn Error + Send + Sync + 'static>>;

    async fn list_public(&self)
        -> Result<Vec<ListArticle>, Box<dyn Error + Send + Sync + 'static>>;
}
