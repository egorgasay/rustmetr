use async_trait::async_trait;

#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait RepositoryAbstract: Send + Sync {
    fn get(&self, metric: String) -> Result<f32, String>;
    fn set(&self, metric: String, value: f32) -> Result<String, String>;
    fn inc(&self, metric: String, value: i32) -> Result<String, String>;
}
//
//#[cfg_attr(test, automock)]
//#[async_trait(?Send)]
//pub trait ValueAbstract: Send + Sync {
//    fn add(&mut self, another: &dyn ValueAbstract);
//}

//
//unsafe impl Sync for dyn CatFactsRepositoryAbstract {}