use config::Config;
use firestore_db_and_auth::Credentials;
use infra::repository::article::ArticleDBRepository;
use std::error::Error;
use usecase::article::post_article::PostArticleUseCase;

pub struct DiContainer {
    credentials: Credentials,
}

impl DiContainer {
    pub fn new(config: Box<Config>) -> Result<Self, Box<dyn Error>> {
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
}
