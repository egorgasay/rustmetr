use async_trait::async_trait;

use crate::{
    application::{repositories::repository_abstract::RepositoryAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{error::ApiError},
};

pub struct UseCase<'a> {
    repository: &'a dyn RepositoryAbstract,
}

impl<'a> UseCase<'a> {
    pub fn new(repository: &'a dyn RepositoryAbstract) -> Self {
        UseCase { repository }
    }

    pub async fn save(&self, metric_name: String, value: i32) -> Option<String> {
        println!("hi from usecase");
        self.repository.save(metric_name, value).await;
        Some("".to_string())
    }
}

