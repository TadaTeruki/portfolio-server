use async_trait::async_trait;
use domain::{
    model::article::{GetArticle, PostArticle, PutArticle},
    repository::article::ArticleRepository,
};
use firestore::*;
use std::error::Error;

pub struct ArticleDBRepository {
    db: FirestoreDb,
}

impl ArticleDBRepository {
    pub fn new(db_: FirestoreDb) -> Self {
        Self { db: db_ }
    }
}

#[async_trait]
impl ArticleRepository for ArticleDBRepository {
    async fn insert(
        &self,
        article: PostArticle,
    ) -> Result<String, Box<dyn Error + Send + Sync + 'static>> {
        let id = article.allocate_new_uuid();

        self.db
            .fluent()
            .insert()
            .into("articles")
            .document_id(id.clone())
            .object(&article)
            .execute()
            .await?;

        Ok(id)
    }

    async fn put(
        &self,
        id: &str,
        article: PutArticle,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.db
            .fluent()
            .update()
            .fields(paths!(PutArticle::{
                title,
                subtitle,
                tags,
                is_public,
                updated_at
            }))
            .in_col("articles")
            .document_id(id)
            .object(&article)
            .execute()
            .await?;
        Ok(())
    }

    async fn get(
        &self,
        id: &str,
    ) -> Result<Option<GetArticle>, Box<dyn Error + Send + Sync + 'static>> {
        let article = self
            .db
            .fluent()
            .select()
            .by_id_in("articles")
            .obj()
            .one(&id)
            .await?;

        Ok(article)
    }

    async fn delete(&self, id: &str) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.db
            .fluent()
            .delete()
            .from("articles")
            .document_id(id)
            .execute()
            .await?;
        Ok(())
    }
}
