use async_trait::async_trait;

#[cfg(test)]
use mockall::{predicate::*, *};

use crate::errors::storage::*;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait RepositoryAbstract: Send + Sync {
    fn get_gauge(&self, name: String) -> Result<f64, GetError>;
    fn get_counter(&self, name: String) -> Result<i64, GetError>;
    fn set_gauge(&self, name: String, value: f64) -> Result<(), SetError>;
    fn inc_counter(&self, name: String, value: i64) -> Result<(), IncError>;
}