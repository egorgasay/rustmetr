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
use std::sync::Mutex;
// Структура для хранения данных
pub struct Storage {
    pub data: Mutex<HashMap<String, f32>>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            data: Mutex::new(HashMap::new()),
        }
    }
}

impl RepositoryAbstract for Storage {
    fn get(&self, metric: String) -> Result<String, String> {
        match self.data.lock().unwrap().get(&metric.to_owned()) {
            Some(value) => Ok(value.to_string()),
            None => Err("metric not found".to_string()),
        }
    }

    fn set(&self, metric: String, value: f32) -> Result<String, String> {
        self.data.lock().unwrap().insert(metric, value);
        Ok("".to_string())
    }
}
