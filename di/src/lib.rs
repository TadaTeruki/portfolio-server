use config::Config;
use firestore_db_and_auth::Credentials;
use infra::repository::article::ArticleDBRepository;
use std::error::Error;
use usecase::article::{
    delete_article::DeleteArticleUseCase, post_article::PostArticleUseCase,
    read_article::ReadArticleUseCase, update_article::UpdateArticleUseCase,
};

pub struct DiContainer {
    credentials: Credentials,
}

impl DiContainer {
    pub fn new(config: Config) -> Result<Self, Box<dyn Error>> {
        let credentials_ = Credentials::from_file(config.get_credentials_src())?;
        Ok(Self {
            credentials: credentials_,
        })
    }

    pub fn usecase_post_article(&self) -> Result<PostArticleUseCase, Box<dyn Error>> {
        let usecase = PostArticleUseCase::new(Box::new(ArticleDBRepository::new(
            self.credentials.clone(),
        )?));
        Ok(usecase)
    }

    pub fn usecase_read_article(&self) -> Result<ReadArticleUseCase, Box<dyn Error>> {
        let usecase = ReadArticleUseCase::new(Box::new(ArticleDBRepository::new(
            self.credentials.clone(),
        )?));
        Ok(usecase)
    }

    pub fn usecase_delete_article(&self) -> Result<DeleteArticleUseCase, Box<dyn Error>> {
        let usecase = DeleteArticleUseCase::new(Box::new(ArticleDBRepository::new(
            self.credentials.clone(),
        )?));
        Ok(usecase)
    }

    pub fn usecase_update_article(&self) -> Result<UpdateArticleUseCase, Box<dyn Error>> {
        let usecase = UpdateArticleUseCase::new(Box::new(ArticleDBRepository::new(
            self.credentials.clone(),
        )?));
        Ok(usecase)
    }
}
