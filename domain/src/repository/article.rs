use crate::model::article::{GetArticle, PostArticle, PutArticle};
use std::error::Error;

pub trait ArticleRepository {
    fn delete(&self, id: &str) -> Result<(), Box<dyn Error>>;
    fn get(&self, id: &str) -> Result<GetArticle, Box<dyn Error>>;
    //fn list(&self) -> Result<Vec<Article>, Box<dyn Error>>;
    fn insert(&self, article: PostArticle) -> Result<String, Box<dyn Error>>;
    fn put(&self, id: &str, article: PutArticle) -> Result<(), Box<dyn Error>>;
}
