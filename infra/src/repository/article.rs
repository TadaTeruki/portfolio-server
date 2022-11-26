use domain::{model::article::PostArticle, repository::article::ArticleRepository};
use firestore_db_and_auth::{documents, Credentials, ServiceSession};
use std::error::Error;

pub struct ArticleDBRepository {
    fs: ServiceSession,
}

impl ArticleDBRepository {
    pub fn new(credentials: Credentials) -> Result<Self, Box<dyn Error>> {
        let repository = Self {
            fs: ServiceSession::new(credentials)?,
        };
        Ok(repository)
    }
}

impl ArticleRepository for ArticleDBRepository {
    fn insert(&self, article: PostArticle) -> Result<(), Box<dyn Error>> {
        documents::write(
            &self.fs,
            "articles",
            Some(article.allocate_new_uuid()),
            &article,
            documents::WriteOptions::default(),
        )?;

        Ok(())
    }
}
