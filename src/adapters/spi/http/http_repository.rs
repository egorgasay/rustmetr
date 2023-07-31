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
    errors::storage::*,
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
    fn get(&self, metric: String) -> Result<f32, GetError> {
        match self.data.lock().unwrap().get(&metric.to_owned()) {
            Some(value) => Ok(*value),
            None => Err(GetError::NotFound),
        }
    }

    fn set(&self, metric: String, value: f32) -> Option<SetError> {
        self.data.lock().unwrap().insert(metric, value);
        None
    }

    fn inc(&self, metric: String, value: i32) -> Option<IncError> {
        let val :f32 = match self.get(metric.clone()) { // TODO: use &metric??
            Ok(v) => v,
            Err(..) => 0.0,
        };

        self.data.lock().unwrap().insert(metric, val + (value as f32));

        None
    }
}
