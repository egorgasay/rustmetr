


use crate::{
    application::{repositories::repository_abstract::RepositoryAbstract,
    },
    errors::storage::*,
};
use std::collections::HashMap;
use std::sync::RwLock;

// Структура для хранения данных
pub struct Storage {
    pub data: RwLock<HashMap<String, f32>>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            data: RwLock::new(HashMap::new()),
        }
    }
}

impl RepositoryAbstract for Storage {
    fn get(&self, metric: String) -> Result<f32, GetError> {
        match self.data.read() {
            Ok(data) => {
                match data.get(&metric.to_owned()) {
                    Some(value) => Ok(*value),
                    None => Err(GetError::NotFound),
                }
            },
            Err(err) => {
                println!("error while getting metric: {}", err.to_string());
                Err(GetError::Internal)
            },
        }
    }

    fn set(&self, metric: String, value: f32) -> Result<(), SetError> {
        println!("set metric: {}, value: {}", metric, value);
        match self.data.write() {
            Ok(mut data) => {
                data.insert(metric, value);
                Ok(())
            }
            Err(err) => {
                println!("error while setting metric: {}", err.to_string());
                Err(SetError::Internal)
            }
        }
    }

    fn inc(&self, metric: String, value: i32) -> Result<(), IncError> {
        let val :f32 = match self.get(metric.clone()) { // TODO: use &metric??
            Ok(v) => v,
            Err(..) => 0.0,
        };
        println!("inc metric: {}, value: {}", metric, val + (value as f32));

        match self.data.write(){
            Ok(mut data) => {
                data.insert(metric, val + (value as f32));
                Ok(())
            }
            Err(err) => {
                println!("error while inc metric: {}", err.to_string());
                Err(IncError::Internal)
            }
        }
    }
}