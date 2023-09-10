use async_trait::async_trait;

#[cfg(test)]
use mockall::{predicate::*, *};
use crate::application::repositories::error::RepositoryError;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait RepositoryAbstract: Send + Sync {
    fn get_gauge(&self, name: &String) -> Result<f64, RepositoryError>;
    fn get_counter(&self, name: &String) -> Result<i64, RepositoryError>;
    fn set_gauge(&self, name: String, value: f64) -> Result<(), RepositoryError>;
    fn inc_counter(&self, name: String, value: i64) -> Result<(), RepositoryError>;
    fn get_all_metrics(&self) -> Result<Vec<(String, f64)>, RepositoryError>;
}