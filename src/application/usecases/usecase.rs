

use crate::{
    application::repositories::repository_abstract::RepositoryAbstract,
    errors::{logic::{*}, storage},
};

#[derive(Clone)]
pub struct UseCase<'a> {
    repository: &'a dyn RepositoryAbstract,
}


impl<'a> UseCase<'_> {
    pub fn new(st: &'a impl RepositoryAbstract) -> UseCase<'a> {
        UseCase {
            repository: st,
        }
    }

    pub fn get_metric(&self, metric_type: String, name: String) -> Result<f64, GetMetricError> {
        match metric_type.as_str() {
            "gauge" => {
                match self.repository.get_gauge(name) {
                    Ok(value) => Ok(value),
                    Err(err) => match err {
                        storage::GetError::NotFound => Err(GetMetricError::NotFound),
                        storage::GetError::Internal => Err(GetMetricError::ProblemStorage),
                    },
                }
            }
            "counter" => {
                match self.repository.get_counter(name) {
                    Ok(value) => Ok(value as f64),
                    Err(err) => match err {
                        storage::GetError::NotFound => Err(GetMetricError::NotFound),
                        storage::GetError::Internal => Err(GetMetricError::ProblemStorage),
                    },
                }
            }
            _ => {
                Err(GetMetricError::UnknownMetric)
            }
        }
    }

    fn update_gauge(&self, metric: String, value: f64) -> Result<(), UpdateError> {
        match self.repository.set_gauge(metric, value) {
            Err(err) => {
                match err {
                    storage::SetError::Internal => Err(UpdateError::ProblemStorage)
                }
            },
            Ok(_) => Ok(()),
        }
    }

    fn update_counter(&self, metric: String, value: i64) -> Result<(), UpdateError> {
        match self.repository.inc_counter(metric, value) {
            Err(err) => {
                match err {
                    storage::IncError::Internal => Err(UpdateError::ProblemStorage),
                }
            }
            Ok(_) => Ok(()),
        }
    }

    pub fn update(&self, metric: String, name: String, value: String) -> Result<(), UpdateError> {
        match metric.as_str() {
            "gauge" => {
                let val: f64;
                match value.parse::<f64>() {
                    Ok(n) => val = n,
                    Err(_e) => {
                        return Err(UpdateError::BadFormat);
                    },
                };

                match self.update_gauge(name, val) {
                    Err(err) => Err(err),
                    Ok(_) => Ok(()),
                }
            },
            "counter" => {
                let val: i64;
                match value.parse::<i64>() {
                    Ok(n) => val = n,
                    Err(_e) => {
                        return Err(UpdateError::BadFormat);
                    },
                };

                match self.update_counter(name, val) {
                    Err(err) => Err(err),
                    Ok(_) => Ok(()),
                }
            },
            &_ => {
                return Err(UpdateError::UnknownMetric);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::application::repositories::repository_abstract::*;

    use super::*;

    #[test]
    fn test_update_storage_error() {
        let mut repo = MockRepositoryAbstract::new();

        repo.expect_set().return_once(move |_name, _value | Err(storage::SetError::Internal));
         
        let usecase = UseCase::new(&repo);
        
        match usecase.update("gauge".to_string(), "name".to_string(), "1.33".to_string()) {
            Ok(_) => {
                panic!("error was expected")
            },
            Err(err) => {
                match err {
                    UpdateError::ProblemStorage => {},
                    _ => {
                       panic!("UpdateError::ProblemStorage error was expected")
                    }
                } 
            }
        }
    }

    #[test]
    fn test_update_unknown_metric_error() {
        let repo = MockRepositoryAbstract::new();
        let usecase = UseCase::new(&repo);

        match usecase.update("qfg".to_string(), "name".to_string(), "1.33".to_string()) {
            Ok(_) => {
                panic!("error was expected")
            },
            Err(err) => {
                match err {
                    UpdateError::UnknownMetric => {},
                    _ => {
                       panic!("UpdateError::UnknownMetric error was expected")
                    }
                } 
            }
        }
    }

    #[test]
    fn test_update_ok_gauge() {
        let mut repo = MockRepositoryAbstract::new();

        repo.expect_set().return_once(move |_, _ | Ok(()) );
         
        let usecase = UseCase::new(&repo);
        
        match usecase.update("gauge".to_string(), "name".to_string(), "1.33".to_string()) {
            Ok(_) => {},
            Err(_) => {
                panic!("no error was expected")
            }
        }
    }

    #[test]
    fn test_update_ok_counter() {
        let mut repo = MockRepositoryAbstract::new();

        repo.expect_inc().return_once(move |_, _ | Ok(()) );

        let usecase = UseCase::new(&repo);

        match usecase.update("counter".to_string(), "name".to_string(), "1".to_string()) {
            Ok(_) => {},
            Err(_) => {
                panic!("no error was expected")
            }
        }
    }

    #[test]
    fn test_update_bad_format_counter() {
        let repo = MockRepositoryAbstract::new();
        let usecase = UseCase::new(&repo);

        match usecase.update("counter".to_string(), "name".to_string(), "1wfe".to_string()) {
            Ok(_) => {
                panic!("error was expected")
            },
            Err(err) => {
                match err {
                    UpdateError::BadFormat => {},
                    _ => {
                       panic!("UpdateError::BadFormat error was expected")
                    }
                } 
            }
        }
    }

    #[test]
    fn test_get_metric() {
        let mut repo = MockRepositoryAbstract::new();

        repo.expect_get().return_once(move |_ | Ok(3.534) );

        let usecase = UseCase::new(&repo);
        match usecase.get_metric("name".to_string()) {
            Ok(f) => {
                assert_eq!(f, 3.534);
            },
            Err(err) => {
                panic!("no error was expected {:?}", err)
            }
        }
    }

    #[test]
    fn test_get_metric_not_found() {
        let mut repo = MockRepositoryAbstract::new();

        repo.expect_get().return_once(move  |name |
            if name != "name" {
                panic!("want param {} got {}", "name", name)
            } else {
                Err(storage::GetError::NotFound) 
            }
        );
        
        let usecase = UseCase::new(&repo);
        match usecase.get_metric("name".to_string()) {
            Ok(_) => {
                panic!("error was expected");
            },
            Err(err) => {
                match err {
                    GetMetricError::NotFound => {},
                    other_error => {
                       panic!("GetMetricError::NotFound error was expected got {:?}", other_error)
                    }
                } 
            }
        }
    }

    #[test]
    fn test_get_metric_storage_error() {
        let mut repo = MockRepositoryAbstract::new();

        repo.expect_get().return_once(move  |name |
            if name != "name" {
                panic!("want param {} got {}", "name", name)
            } else {
                Err(storage::GetError::Internal) 
            }
        );
        
        let usecase = UseCase::new(&repo);
        match usecase.get_metric("name".to_string()) {
            Ok(_) => {
                panic!("error was expected");
            },
            Err(err) => {
                match err {
                    GetMetricError::ProblemStorage => {},
                    other_error => {
                       panic!("GetMetricError::ProblemStorage error was expected got {:?}", other_error)
                    }
                } 
            }
        }
    }
}