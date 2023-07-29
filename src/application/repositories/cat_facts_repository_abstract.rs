use async_trait::async_trait;

use crate::domain::cat_fact_entity::CatFactEntity;

#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait CatFactsRepositoryAbstract: Send + Sync {
    async fn get_random_cat_fact(&self) -> Result<CatFactEntity, Box<dyn Error>>;
    async fn get_all_cat_facts(&self) -> Result<Vec<CatFactEntity>, Box<dyn Error>>;
    async fn save(&self, metric_name: String, value: i32) -> Option<String>;
}
//
//unsafe impl Sync for dyn CatFactsRepositoryAbstract {}