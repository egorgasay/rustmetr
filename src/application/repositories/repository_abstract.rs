use async_trait::async_trait;

#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;
use crate::errors::storage::*;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait RepositoryAbstract: Send + Sync {
    fn get(&self, metric: String) -> Result<f32, GetError>;
    fn set(&self, metric: String, value: f32) -> Option<SetError>;
    fn inc(&self, metric: String, value: i32) -> Option<IncError>;
}
//
//#[cfg_attr(test, automock)]
//#[async_trait(?Send)]
//pub trait ValueAbstract: Send + Sync {
//    fn add(&mut self, another: &dyn ValueAbstract);
//}

//
//unsafe impl Sync for dyn CatFactsRepositoryAbstract {}