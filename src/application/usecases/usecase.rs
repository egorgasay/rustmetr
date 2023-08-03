use async_trait::async_trait;

use crate::{
    application::{
        repositories::repository_abstract::RepositoryAbstract,
        usecases::interfaces::AbstractUseCase,
        utils::error_handling_utils::ErrorHandlingUtils,
    },
    domain::{error::ApiError},
    errors::logic::*,
    errors::storage::SetError,
    errors::storage::GetError,
};

#[derive(Clone)]
pub struct UseCase<'a> {
    repository: &'a dyn RepositoryAbstract,
}


impl<'a> UseCase<'_> {
    pub fn new(st: &'a dyn RepositoryAbstract) -> UseCase<'a> {
        UseCase {
            repository: st,
        }
    }

    pub fn get_metric(&self, metric: String) -> Result<f32, GetMetricError> {
        match self.repository.get(metric) {
            Ok(value) => Ok(value),
            Err(err) => match err {
                GetError::NotFound => Err(GetMetricError::NotFound),
            },
        }
    }

    fn update_gauge(&self, metric: String, value: f32) -> Option<UpdateError> {
        match self.repository.set(metric, value) {
            Some(err) => {
                match err {
                    ProblemStorage => Some(UpdateError::ProblemStorage)
                }
            },
            None => None,
        }
    }

    fn update_counter(&self, metric: String, value: i32) -> Option<UpdateError> {
        match self.repository.inc(metric, value) {
            Some(err) => {
                match err {
                    ProblemStorage => Some(UpdateError::ProblemStorage)
                }
            }
            None => None,
        }
    }

    pub fn update(&self, metric: String, name: String, value: String) -> Option<UpdateError> {
        match metric.as_str() {
            "gauge" => {
                let mut val: f32 = 0 as f32;
                match value.parse::<f32>() {
                    Ok(n) => val = n,
                    Err(e) => {
                        return Some(UpdateError::BadFormat);
                    },
                };

                match self.update_gauge(name, val) {
                    Some(err) => Some(err),
                    None => None,
                }
            },
            "counter" => {
                let mut val: i32 = 0;
                match value.parse::<i32>() {
                    Ok(n) => val = n,
                    Err(e) => {
                        return Some(UpdateError::BadFormat);
                    },
                };

                match self.update_counter(name, val) {
                    Some(err) => Some(err),
                    None => None,
                }
            },
            &_ => Some(UpdateError::UnknownMetric)
        }
    }
}
