use domain::{
    model::article::{GetArticle, PostArticle, PutArticle},
    repository::article::ArticleRepository,
};
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
    fn insert(&self, article: PostArticle) -> Result<String, Box<dyn Error>> {
        let id = article.allocate_new_uuid();
        let res = id.clone();

        documents::write(
            &self.fs,
            "articles",
            Some(id),
            &article,
            documents::WriteOptions::default(),
        )?;

        Ok(res)
    }

    fn put(&self, id: &str, article: PutArticle) -> Result<(), Box<dyn Error>> {
        documents::write(
            &self.fs,
            "articles",
            Some(id),
            &article,
            documents::WriteOptions { merge: true },
        )?;

        Ok(())
    }

    fn get(&self, id: &str) -> Result<GetArticle, Box<dyn Error>> {
        let article = documents::read(&self.fs, "articles", id)?;
        Ok(article)
    }

    fn delete(&self, id: &str) -> Result<(), Box<dyn Error>> {
        documents::delete(&self.fs, &("articles/".to_owned() + id), false)?;
        Ok(())
    }
}
