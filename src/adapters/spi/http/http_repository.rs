use async_trait::async_trait;
use std::error::Error;

use crate::{
    adapters::spi::http::{
        http_connection::HttpConnection,
       // http_mappers::HTTPMapper,
        // http_models::{CatFactApiModel, CatFactsApiModel},
    },
    application::{mappers::http_mapper::HttpMapper,
        repositories::repository_abstract::RepositoryAbstract,
    },
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Repository {
    pub http_connection: HttpConnection,
    pub source: String,
    pub map: HashMap<String, (i32, i32)>,
}

#[async_trait(?Send)]
impl RepositoryAbstract for Repository {
    fn save(&mut self, metric_name: String, value: (i32, i32)) -> Option<String> {
        println!("saved in {}", metric_name);
        self.map.insert(metric_name, value);
        None
    }
}
