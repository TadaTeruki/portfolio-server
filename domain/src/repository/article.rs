use crate::model::article::{GetArticle, PostArticle};
use std::error::Error;

pub trait ArticleRepository {
    fn get(&self, id: &str) -> Result<GetArticle, Box<dyn Error>>;
    //fn list(&self) -> Result<Vec<Article>, Box<dyn Error>>;
    fn insert(&self, article: PostArticle) -> Result<String, Box<dyn Error>>;
    //async fn update(&self, article: &Article) -> Result<(), >;
}
