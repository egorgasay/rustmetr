
use crate::application::service::metric::MetricService;


use actix_web::{get, post, web, HttpResponse, Responder};
use crate::adapters::api::error_presenter::ErrorResponse;


pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.
        service(get_metric).
        service(update).
        service(get_all);
}

#[get("/value/{type}/{name}")]
async fn get_metric(logic: web::Data<MetricService<'_>>, p: web::Path<(String, String)>) -> impl Responder {
    let path = p.into_inner();
    let metric_type = path.0;
    let key = path.1;

    match logic.get_metric(metric_type, key) {
        Ok(value) => HttpResponse::Ok().body(value.to_string()),
        Err(err) => HttpResponse::from_error(ErrorResponse::from(err)),
    }
}

#[post("/update/{type}/{key}/{value}")]
async fn update(logic: web::Data<MetricService<'_>>, path: web::Path<(String, String, String)>) -> impl Responder {
    let p = path.into_inner();
    let metric_type = p.0;
    let key = p.1;
    let value = p.2;

    match logic.update(metric_type, key, value) {
        Ok(_) => HttpResponse::Ok().body("completed successfully"),
        Err(err) => HttpResponse::from_error(ErrorResponse::from(err)),
    }
}

#[get("/")]
async fn get_all(logic: web::Data<MetricService<'_>>) -> impl Responder {
    match logic.get_all_metrics() {
        Ok(value) => HttpResponse::Ok().body(value.to_string()),
        Err(err) => HttpResponse::from_error(ErrorResponse::from(err)),
    }
}