
use crate::application::service::metric::MetricService;


use actix_web::{get, post, web, HttpResponse, Responder};
use log::{Level, log};
use crate::adapters::api::error_presenter::ErrorResponse;
use crate::adapters::spi::http::http_mappers::HTTPMapper;
use crate::adapters::spi::http::http_models::{MetricAPI};
use crate::application::mappers::http_mapper::HttpMapper;
use crate::application::service::errors::ServiceError;


pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.
        service(get_metric).
        service(update).
        service(get_all).
        service(get_metric_json).
        service(update_json);
}

#[get("/value/{type}/{name}")]
async fn get_metric(logic: web::Data<MetricService<'_>>, p: web::Path<(String, String)>) -> impl Responder {
    let path = p.into_inner();
    let metric_type = path.0;
    let key = path.1;

    match logic.get_metric(metric_type.clone(), key) {
        Ok(m) => {
            let body: String;
            match metric_type.as_str() {
                "gauge" => body = m.value.unwrap_or_default().to_string(),
                "counter" => body = m.delta.unwrap_or_default().to_string(),
                _ => return HttpResponse::from_error(ErrorResponse::from(ServiceError::BadRequest("unknown metric".to_string())))
            }
            HttpResponse::Ok().body(body)
        },
        Err(err) => HttpResponse::from_error(ErrorResponse::from(err)),
    }
}

#[post("/value")]
async fn get_metric_json(logic: web::Data<MetricService<'_>>, metric: web::Json<MetricAPI>) -> HttpResponse {
    match logic.get_metric(metric.mtype.clone(), metric.id.clone()) {
        Ok(m) => HttpResponse::Ok().json(HTTPMapper::to_http(m)),
        Err(err) => HttpResponse::from_error(ErrorResponse::from(err)),
    }
}

#[post("/update/{type}/{key}/{value}")]
async fn update(logic: web::Data<MetricService<'_>>, path: web::Path<(String, String, String)>) -> impl Responder {
    let p = path.into_inner();
    let metric_type = p.0;
    let key = p.1;
    let value = p.2;

    match logic.update_raw(metric_type, key, value) {
        Ok(_) => HttpResponse::Ok().body("completed successfully"),
        Err(err) => HttpResponse::from_error(ErrorResponse::from(err)),
    }
}

#[post("/update")]
async fn update_json(logic: web::Data<MetricService<'_>>, metric: web::Json<MetricAPI>) -> impl Responder {
    let mut value= 0.0;
    if let Some(v) = metric.value {
        value = v;
    }

    let mut delta = 0.0;
    if let Some(d) = metric.delta {
        delta = d as f64;
    }

    let m = HTTPMapper::to_entity(metric);

    match logic.update(m) {
        Ok(_) => HttpResponse::Ok().body("completed successfully"),
        Err(err) => HttpResponse::from_error(ErrorResponse::from(err)),
    }
}

#[get("/")]
async fn get_all(tmpl: web::Data<tera::Tera>, logic: web::Data<MetricService<'_>>) -> impl Responder {
    match logic.get_all_metrics() {
        Ok(metrics) => {
            let mut ctx = tera::Context::new();
            ctx.insert("metrics", &metrics);
            match tmpl.render("index.html", &ctx) {
                Ok(html) => HttpResponse::Ok().body(html),
                Err(err) => {
                    log!(Level::Error, "error while rendering template: {}", err.to_string());
                    HttpResponse::from_error(ErrorResponse::from(ServiceError::InternalServerError))
                },
            }
        },
        Err(err) => HttpResponse::from_error(ErrorResponse::from(err)),
    }
}