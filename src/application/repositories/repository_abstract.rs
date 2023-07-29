use async_trait::async_trait;

#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait RepositoryAbstract: Send + Sync {
    async fn save(&self, metric_name: String, value: i32) -> Option<String>;
}
//
//unsafe impl Sync for dyn CatFactsRepositoryAbstract {}