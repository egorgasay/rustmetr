
use crate::application::usecases::usecase::UseCase;
use crate::{
    errors::logic::*,
};


use actix_web::{get, post, web, HttpResponse, Responder};


pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_metric).service(update);
}

// Контроллерные функции для обработки запросов
#[get("/get/{key}")]
async fn get_metric(logic: web::Data<UseCase<'_>>, k: web::Path<(String,)>) -> impl Responder {
    let kk = k.into_inner();
    match logic.get_metric(kk.0) {
        Ok(value) => HttpResponse::Ok().body(value.to_string()),
        Err(err) => match err {
            GetMetricError::NotFound => HttpResponse::NotFound().body("err"),
            GetMetricError::ProblemStorage => HttpResponse::InternalServerError().body("internal server error")
        }
    }
}

#[post("/update/{metric}/{key}/{value}")]
async fn update(logic: web::Data<UseCase<'_>>, path: web::Path<(String,String,String)>) -> impl Responder {
    let p = path.into_inner();
    let metric = p.0;
    let key = p.1;
    let value = p.2;

    match logic.update(metric, key, value) {
        Ok(_) => HttpResponse::Ok().body("completed successfully"),
        Err(err) => {
            match err {
                UpdateError::UnknownMetric => {
                    HttpResponse::BadRequest().body("unknown metric")
                }
                UpdateError::NotFound => {
                    HttpResponse::NotFound().body("metric was not found")
                }
                UpdateError::BadFormat => {
                    HttpResponse::BadRequest().body("bad request")
                }
                UpdateError::ProblemStorage => {
                    HttpResponse::InternalServerError().body("internal server error")
                }
            }
        },
    }
}
