use async_trait::async_trait;

use crate::{
    application::{
        repositories::repository_abstract::RepositoryAbstract,
        usecases::interfaces::AbstractUseCase,
        utils::error_handling_utils::ErrorHandlingUtils,
    },
    domain::{error::ApiError},
};

#[derive(Clone)]
pub struct UseCase<'a> {
    repository: &'a dyn RepositoryAbstract,
}


impl<'a> UseCase<'_> {
    pub fn new(st: &'a dyn RepositoryAbstract) -> UseCase<'a> {
        UseCase {
            repository: st,
        }
    }

    pub fn get_metric(&self, metric: String) -> Result<f32, String> {
        match self.repository.get(metric) {
            Ok(value) => Ok(value),
            Err(err) => Err(err),
        }
    }

    pub fn update_gauge(&self, metric: String, value: f32) -> Result<String, String> {
        match self.repository.set(metric, value) {
            Ok(value) => Ok(value),
            Err(err) => Err(err),
        }
    }

    pub fn update_counter(&self, metric: String, value: i32) -> Result<String, String> {
        match self.repository.inc(metric, value) {
            Ok(value) => Ok(value),
            Err(err) => Err(err),
        }
    }
}
