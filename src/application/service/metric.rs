use log::{Level, log};
use crate::{
    application::repositories::repository_abstract::RepositoryAbstract,
};
use crate::application::repositories::errors::RepositoryError;
use crate::application::service::errors::ServiceError;

#[derive(Clone)]
pub struct MetricService<'a> {
    repository: &'a dyn RepositoryAbstract,
}


impl<'a> MetricService<'_> {
    pub fn new(st: &'a impl RepositoryAbstract) -> MetricService<'a> {
        MetricService {
            repository: st,
        }
    }

    pub fn get_metric(&self, metric_type: String, name: String) -> Result<f64, ServiceError> {
        match metric_type.as_str() {
            "gauge" => {
                match self.repository.get_gauge(name) {
                    Ok(value) => Ok(value),
                    Err(err) => match err {
                        RepositoryError::NotFound => Err(ServiceError::NotFound),
                        RepositoryError::Internal => Err(ServiceError::InternalServerError),
                    },
                }
            }
            "counter" => {
                match self.repository.get_counter(name) {
                    Ok(value) => Ok(value as f64),
                    Err(err) => match err {
                        RepositoryError::NotFound => Err(ServiceError::NotFound),
                        RepositoryError::Internal => Err(ServiceError::InternalServerError),
                    },
                }
            }
            _ => {
                Err(ServiceError::BadRequest("unknown metric".to_string()))
            }
        }
    }

    fn update_gauge(&self, metric: String, value: f64) -> Result<(), ServiceError> {
        match self.repository.set_gauge(metric, value) {
            Err(err) => {
                match err {
                    RepositoryError::Internal => Err(ServiceError::InternalServerError),
                    _ => {
                        log!(Level::Error, "error while updating gauge: {}", err.to_string());
                        Err(ServiceError::InternalServerError)
                    },
                }
            },
            Ok(_) => Ok(()),
        }
    }

    fn update_counter(&self, metric: String, value: i64) -> Result<(), ServiceError> {
        match self.repository.inc_counter(metric, value) {
            Err(err) => {
                match err {
                    RepositoryError::Internal => Err(ServiceError::InternalServerError),
                    _ => {
                        log!(Level::Error, "error while updating counter: {}", err.to_string());
                        Err(ServiceError::InternalServerError)
                    }
                }
            }
            Ok(_) => Ok(()),
        }
    }

    pub fn update(&self, metric: String, name: String, value: String) -> Result<(), ServiceError> {
        match metric.as_str() {
            "gauge" => {
                let val;
                match value.parse::<f64>() {
                    Ok(n) => val = n,
                    Err(_e) => {
                        return Err(ServiceError::BadRequest(_e.to_string()));
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
                        return Err(ServiceError::BadRequest(_e.to_string()));
                    },
                };

                match self.update_counter(name, val) {
                    Err(err) => Err(err),
                    Ok(_) => Ok(()),
                }
            },
            &_ => {
                Err(ServiceError::BadRequest("unknown metric".to_string()))
            }
        }
    }

    pub fn get_all_metrics(&self) -> Result<Vec<(String, f64)>, ServiceError> {
        match self.repository.get_all_metrics() {
            Ok(m) => Ok(m),
            Err(err) => Err(ServiceError::from(err)),
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

        repo.expect_set_gauge().return_once(move |_name, _value | Err(RepositoryError::Internal));

        let usecase = MetricService::new(&repo);

        match usecase.update("gauge".to_string(), "name".to_string(), "1.33".to_string()) {
            Ok(_) => {
                panic!("error was expected")
            },
            Err(err) => {
                match err {
                    ServiceError::InternalServerError => {},
                    _ => {
                       panic!("ServiceError::InternalServerError error was expected")
                    }
                }
            }
        }
    }

    #[test]
    fn test_update_unknown_metric_error() {
        let repo = MockRepositoryAbstract::new();
        let usecase = MetricService::new(&repo);

        match usecase.update("qfg".to_string(), "name".to_string(), "1.33".to_string()) {
            Ok(_) => {
                panic!("error was expected")
            },
            Err(err) => {
                match err {
                    ServiceError::BadRequest(_e) => {},
                    _ => {
                       panic!("ServiceError::BadRequest error was expected")
                    }
                }
            }
        }
    }

    #[test]
    fn test_update_ok_gauge() {
        let mut repo = MockRepositoryAbstract::new();

        repo.expect_set_gauge().return_once(move |_, _ | Ok(()) );

        let usecase = MetricService::new(&repo);

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

        repo.expect_inc_counter().return_once(move |_, _ | Ok(()) );

        let usecase = MetricService::new(&repo);

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
        let usecase = MetricService::new(&repo);

        match usecase.update("counter".to_string(), "name".to_string(), "1wfe".to_string()) {
            Ok(_) => {
                panic!("error was expected")
            },
            Err(err) => {
                match err {
                    ServiceError::BadRequest(_e) => {},
                    _ => {
                       panic!("ServiceError::BadRequest error was expected")
                    }
                } 
            }
        }
    }

    #[test]
    fn test_getg_metric() {
        let mut repo = MockRepositoryAbstract::new();

        repo.expect_get_gauge().return_once(move |_ | Ok(3.534) );

        let usecase = MetricService::new(&repo);
        match usecase.get_metric("gauge".to_string(), "name".to_string()) {
            Ok(f) => {
                assert_eq!(f, 3.534);
            },
            Err(err) => {
                panic!("no error was expected {:?}", err)
            }
        }
    }

    #[test]
    fn test_getc_metric() {
        let mut repo = MockRepositoryAbstract::new();

        repo.expect_get_counter().return_once(move |_ | Ok(3) );

        let usecase = MetricService::new(&repo);
        match usecase.get_metric("counter".to_string(), "name".to_string()) {
            Ok(f) => {
                assert_eq!(f, 3.0);
            },
            Err(err) => {
                panic!("no error was expected {:?}", err)
            }
        }
    }

    #[test]
    fn test_get_gauge_metric_not_found() {
        let mut repo = MockRepositoryAbstract::new();

        repo.expect_get_gauge().return_once(move  |name |
            if name != "name" {
                panic!("want param {} got {}", "name", name)
            } else {
                Err(RepositoryError::NotFound)
            }
        );
        
        let usecase = MetricService::new(&repo);
        match usecase.get_metric("gauge".to_string(), "name".to_string()) {
            Ok(_) => {
                panic!("error was expected");
            },
            Err(err) => {
                match err {
                    ServiceError::NotFound => {},
                    other_error => {
                       panic!("ServiceError::NotFound error was expected got {:?}", other_error)
                    }
                } 
            }
        }
    }


    #[test]
    fn test_get_counter_metric_not_found() {
        let mut repo = MockRepositoryAbstract::new();

        repo.expect_get_counter().return_once(move  |name |
            if name != "name" {
                panic!("want param {} got {}", "name", name)
            } else {
                Err(RepositoryError::NotFound)
            }
        );

        let usecase = MetricService::new(&repo);
        match usecase.get_metric("counter".to_string(), "name".to_string()) {
            Ok(_) => {
                panic!("error was expected");
            },
            Err(err) => {
                match err {
                    ServiceError::NotFound => {},
                    other_error => {
                        panic!("ServiceError::NotFound error was expected got {:?}", other_error)
                    }
                }
            }
        }
    }

    #[test]
    fn test_get_metric_storage_error() {
        let mut repo = MockRepositoryAbstract::new();

        repo.expect_get_counter().return_once(move  |name |
            if name != "name" {
                panic!("want param {} got {}", "name", name)
            } else {
                Err(RepositoryError::Internal)
            }
        );
        
        let usecase = MetricService::new(&repo);
        match usecase.get_metric("counter".to_string(), "name".to_string()) {
            Ok(_) => {
                panic!("error was expected");
            },
            Err(err) => {
                match err {
                    ServiceError::InternalServerError => {},
                    other_error => {
                       panic!("ServiceError::InternalServerError error was expected got {:?}", other_error)
                    }
                } 
            }
        }
    }
}