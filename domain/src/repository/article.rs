use crate::model::article::PostArticle;
use std::error::Error;

pub trait ArticleRepository {
    //async fn query(&self, id: &ArticleId) -> Result<Article, Box<dyn Error>>;
    //async fn list(&self) -> Result<Vec<Article>, >;
    fn insert(&self, article: PostArticle) -> Result<(), Box<dyn Error>>;
    //async fn update(&self, article: &Article) -> Result<(), >;
}
