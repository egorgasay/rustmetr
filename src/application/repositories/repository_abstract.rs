use async_trait::async_trait;

#[cfg(test)]
use mockall::{predicate::*, *};

use crate::errors::storage::*;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait RepositoryAbstract: Send + Sync {
    fn get(&self, metric: String) -> Result<f32, GetError>;
    fn set(&self, metric: String, value: f32) -> Result<(), SetError>;
    fn inc(&self, metric: String, value: i32) -> Result<(), IncError>;
}