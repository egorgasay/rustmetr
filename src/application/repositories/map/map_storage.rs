


use crate::{
    application::{repositories::repository_abstract::RepositoryAbstract,
    },
};
use std::collections::HashMap;
use std::sync::RwLock;
use log::{Level, log};
use crate::application::repositories::errors::RepositoryError;

// Структура для хранения данных
pub struct Storage {
    pub gauge: RwLock<HashMap<String, f64>>,
    pub counter: RwLock<HashMap<String, i64>>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            gauge: RwLock::new(HashMap::new()),
            counter: RwLock::new(HashMap::new()),
        }
    }
}

impl RepositoryAbstract for Storage {
    fn get_gauge(&self, metric: String) -> Result<f64, RepositoryError> {
        match self.gauge.read() {
            Ok(data) => {
                match data.get(&metric.to_owned()) {
                    Some(value) => Ok(*value),
                    None => Err(RepositoryError::NotFound),
                }
            },
            Err(err) => {
                log!(Level::Error, "error while getting gauge: {}", err.to_string());
                Err(RepositoryError::Internal)
            },
        }
    }

    fn get_counter(&self, metric: String) -> Result<i64, RepositoryError> {
        match self.counter.read() {
            Ok(data) => {
                match data.get(&metric.to_owned()) {
                    Some(value) => Ok(*value),
                    None => Err(RepositoryError::NotFound),
                }
            },
            Err(err) => {
                log!(Level::Error, "error while getting counter: {}", err.to_string());
                Err(RepositoryError::Internal)
            },
        }
    }

    fn set_gauge(&self, name: String, value: f64) -> Result<(), RepositoryError> {
        log!(Level::Info, "set gauge: {}, value: {}", name, value);
        match self.gauge.write() {
            Ok(mut data) => {
                data.insert(name, value);
                Ok(())
            }
            Err(err) => {
                log!(Level::Error, "error while setting gauge: {}", err.to_string());
                Err(RepositoryError::Internal)
            }
        }
    }

    fn inc_counter(&self, name: String, value: i64) -> Result<(), RepositoryError> {
        match self.counter.write(){
            Ok(mut data) => {
                log!(Level::Info, "inc counter: {}, with value: {}", name, value);
                *data.entry(name).or_insert(value) += value;
                Ok(())
            }
            Err(err) => {
                log!(Level::Error, "inc counter: {}", err.to_string());
                Err(RepositoryError::Internal)
            }
        }
    }
}