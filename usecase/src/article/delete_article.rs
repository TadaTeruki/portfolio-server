use domain::repository::article::ArticleRepository;
use std::error::Error;

pub struct DeleteArticleUseCase {
    repository: Box<dyn ArticleRepository>,
}

impl DeleteArticleUseCase {
    pub fn new(repository_: Box<dyn ArticleRepository>) -> Self {
        Self {
            repository: repository_,
        }
    }

    pub fn execute(&self, id: &str) -> Result<(), Box<dyn Error>> {
        self.repository.delete(id)
    }
}
