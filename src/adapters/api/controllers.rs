
use crate::application::usecases::usecase::UseCase;
use crate::{
    errors::logic::*,
};


use actix_web::{get, post, web, HttpResponse, Responder};


pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_metric).service(update);
}

// Контроллерные функции для обработки запросов
#[get("/value/{type}/{name}")]
async fn get_metric(logic: web::Data<UseCase<'_>>, p: web::Path<(String,String)>) -> impl Responder {
    let path = p.into_inner();
    let metric_type = path.0;
    let key = path.1;

    match logic.get_metric(metric_type, key) {
        Ok(value) => HttpResponse::Ok().body(value.to_string()),
        Err(err) => match err {
            GetMetricError::NotFound => HttpResponse::NotFound().body("not found"),
            GetMetricError::UnknownMetric => HttpResponse::BadRequest().body("unknown metric"),
            GetMetricError::ProblemStorage => HttpResponse::InternalServerError().body("internal server error")
        }
    }
}

#[post("/update/{type}/{key}/{value}")]
async fn update(logic: web::Data<UseCase<'_>>, path: web::Path<(String,String,String)>) -> impl Responder {
    let p = path.into_inner();
    let metric_type = p.0;
    let key = p.1;
    let value = p.2;

    match logic.update(metric_type, key, value) {
        Ok(_) => HttpResponse::Ok().body("completed successfully"),
        Err(err) => {
            match err {
                UpdateError::UnknownMetric => HttpResponse::BadRequest().body("unknown metric"),
                UpdateError::BadFormat => HttpResponse::BadRequest().body("bad request"),
                UpdateError::ProblemStorage =>  HttpResponse::InternalServerError().body("internal server error"),
            }
        },
    }
}
