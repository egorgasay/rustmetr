use async_trait::async_trait;
use std::error::Error;

use crate::{
    adapters::spi::http::{
        http_connection::HttpConnection,
       // http_mappers::HTTPMapper,
        // http_models::{CatFactApiModel, CatFactsApiModel},
    },
    application::{mappers::http_mapper::HttpMapper, repositories::repository_abstract::RepositoryAbstract},
};

pub struct Repository {
    pub http_connection: HttpConnection,
    pub source: String,
}

#[async_trait(?Send)]
impl RepositoryAbstract for Repository {
    async fn save(&self, metric_name: String, value: i32) -> Option<String> {
        println!("saved in {}: {}", metric_name, value);

        None
    }
}
