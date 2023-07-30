use async_trait::async_trait;

use crate::{
    application::{
        repositories::repository_abstract::RepositoryAbstract,
        usecases::interfaces::AbstractUseCase,
        utils::error_handling_utils::ErrorHandlingUtils,
    },
    domain::{error::ApiError},
};

pub struct UseCase<'a> {
    repository: &'a mut dyn RepositoryAbstract,
}

//pub struct Counter(i32, i32);
//
//impl Counter {
//    pub fn new(first: i32, secound: i32) -> Self {
//        Counter(first, secound)
//    }
//
//    pub fn add(&mut self, another: &dyn ValueAbstract) {
//        self.0 += another.0;
//        self.1 += another.1;
//    }
//}
//
//pub struct Gauge(i32);
//
//impl Gauge {
//    pub fn new(num: i32) -> Self {
//        Gauge(num)
//    }
//
//    pub fn add(&mut self, another: &Gauge) {
//        self.0 = another.0;
//    }
//}


impl<'a> UseCase<'a> {
    pub fn new(repository: &'a mut dyn RepositoryAbstract) -> Self {
        UseCase { repository }
    }

    pub fn update_gauge(&mut self, metric_name: String, value: i32) -> Option<String> {
        println!("hi from update_gauge");
        self.repository.save(metric_name, (value, 0));
        Some("".to_string())
    }

    pub fn update_counter(&mut self, metric_name: String, value: String) -> Option<String> {
        println!("hi from update_counter");

        let val = value.split(".").collect::<Vec<&str>>();
        let fpart = val[0].parse::<i32>().unwrap();

        let mut setOfFloat: (i32, i32) = (fpart, 0);

        if val.len() > 1 {
            setOfFloat.1 = val[1].parse::<i32>().unwrap();
            println!("got {:?}", setOfFloat);
        }

        self.repository.save(metric_name, setOfFloat);
        Some("".to_string())
    }
}

