use async_trait::async_trait;

#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait RepositoryAbstract: Send + Sync {
    fn save(&mut self, metric_name: String, value: (i32, i32)) -> Option<String>;
}
//
//#[cfg_attr(test, automock)]
//#[async_trait(?Send)]
//pub trait ValueAbstract: Send + Sync {
//    fn add(&mut self, another: &dyn ValueAbstract);
//}

//
//unsafe impl Sync for dyn CatFactsRepositoryAbstract {}