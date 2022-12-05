use async_trait::async_trait;
use domain::{
    model::article::{GetArticle, ListArticle, PostArticle, PutArticle},
    repository::article::ArticleRepository,
};
use firestore::{path, paths, struct_path, FirestoreDb, FirestoreQueryDirection};
use futures::stream::BoxStream;
use futures::StreamExt;
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
        self.db
            .fluent()
            .insert()
            .into("articles")
            .document_id(article.id.clone())
            .object(&article)
            .execute()
            .await?;

        Ok(article.id)
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
                body,
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

    async fn list(&self) -> Result<Vec<ListArticle>, Box<dyn Error + Send + Sync + 'static>> {
        let stream: BoxStream<ListArticle> = self
            .db
            .fluent()
            .select()
            .fields(
                paths!(ListArticle::{id, title, subtitle, tags, is_public, created_at, updated_at}),
            )
            .from("articles")
            .order_by([(
                path!(ListArticle::created_at),
                FirestoreQueryDirection::Descending,
            )])
            .obj()
            .stream_query()
            .await?;

        let list: Vec<ListArticle> = stream.collect().await;
        Ok(list)
    }

    async fn list_public(
        &self,
    ) -> Result<Vec<ListArticle>, Box<dyn Error + Send + Sync + 'static>> {
        let stream: BoxStream<ListArticle> = self
            .db
            .fluent()
            .select()
            .fields(
                paths!(ListArticle::{id, title, subtitle, tags, is_public, created_at, updated_at}),
            )
            .from("articles")
            .filter(|q| q.for_all([q.field(path!(ListArticle::is_public)).eq(true)]))
            .order_by([(
                path!(ListArticle::created_at),
                FirestoreQueryDirection::Descending,
            )])
            .obj()
            .stream_query()
            .await?;

        let list: Vec<ListArticle> = stream.collect().await;
        Ok(list)
    }
}
