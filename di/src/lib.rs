use config::Config;
use firestore::FirestoreDb;
use infra::repository::{article::ArticleDBRepository, auth::AuthDBRepository};
use std::error::Error;
use usecase::{
    article::{
        delete_article::DeleteArticleUseCase, list_article::ListArticleUseCase,
        post_article::PostArticleUseCase, read_article::ReadArticleUseCase,
        update_article::UpdateArticleUseCase,
    },
    auth::{check_ownership::CheckOwnershipUseCase, login_as_owner::LoginAsOwnerUseCase},
};

pub struct DiContainer {
    db: FirestoreDb,
    pub config: Config,
}

impl DiContainer {
    pub async fn new() -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        let config_ = Config::init()?;
        let db_ = FirestoreDb::new(config_.get_project_id()).await?;
        Ok(Self {
            db: db_,
            config: config_,
        })
    }

    pub fn usecase_post_article(&self) -> PostArticleUseCase {
        PostArticleUseCase::new(Box::new(ArticleDBRepository::new(self.db.clone())))
    }

    pub fn usecase_read_article(&self) -> ReadArticleUseCase {
        ReadArticleUseCase::new(Box::new(ArticleDBRepository::new(self.db.clone())))
    }

    pub fn usecase_delete_article(&self) -> DeleteArticleUseCase {
        DeleteArticleUseCase::new(Box::new(ArticleDBRepository::new(self.db.clone())))
    }

    pub fn usecase_update_article(&self) -> UpdateArticleUseCase {
        UpdateArticleUseCase::new(Box::new(ArticleDBRepository::new(self.db.clone())))
    }

    pub fn usecase_list_article(&self) -> ListArticleUseCase {
        ListArticleUseCase::new(Box::new(ArticleDBRepository::new(self.db.clone())))
    }

    pub fn usecase_login_as_owner(&self) -> LoginAsOwnerUseCase {
        LoginAsOwnerUseCase::new(Box::new(AuthDBRepository::new(self.db.clone())))
    }

    pub fn usecase_check_ownership(&self) -> CheckOwnershipUseCase {
        CheckOwnershipUseCase::new()
    }
}
