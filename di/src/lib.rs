use config::Config;
use firestore::FirestoreDb;
use infra::repository::article::ArticleDBRepository;
use std::error::Error;
use usecase::article::{
    delete_article::DeleteArticleUseCase, list_article::ListArticleUseCase,
    post_article::PostArticleUseCase, read_article::ReadArticleUseCase,
    update_article::UpdateArticleUseCase,
};

pub struct DiContainer {
    db: FirestoreDb,
}

impl DiContainer {
    pub async fn new(config: Config) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        let db = FirestoreDb::new(config.get_project_id()).await?;
        Ok(Self { db })
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
}
