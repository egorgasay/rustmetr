use crate::application::mappers::api_mapper::ApiMapper;
use crate::application::usecases::usecase::UseCase;
use crate::{
    adapters::api::{
        //app_state::UseCase,
        error_presenter::ErrorReponse,
    }
};
use std::sync::{Mutex, RwLock};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::body::BoxBody;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_metric).service(update_gauge).service(update_counter);
}

// Контроллерные функции для обработки запросов
#[get("/get/{key}")]
async fn get_metric(logic: web::Data<UseCase<'_>>, k: web::Path<(String,)>) -> impl Responder {
    let kk = k.into_inner();
    match logic.get_metric(kk.0) {
        Ok(value) => HttpResponse::Ok().body(value),
        Err(err) => HttpResponse::NotFound().body(err),
    }
}

#[post("/update/gauge/{key}/{value}")]
async fn update_gauge(logic: web::Data<UseCase<'_>>, path: web::Path<(String,String)>) -> impl Responder {
    let p = path.into_inner();
    let key = p.0;
    let value: f32 = p.1.parse().unwrap();
    match logic.update_gauge(key, value) {
        Ok(value) => HttpResponse::Ok().body(value),
        Err(err) => HttpResponse::NotFound().body(err),
    }
}

#[post("/update/counter/{key}/{value}")]
async fn update_counter(logic: web::Data<UseCase<'_>>, path: web::Path<(String,String)>) -> impl Responder {
    let p = path.into_inner();
    let key = p.0;
    let split = p.1.split(".");
    if split.collect::<Vec<_>>().len() > 1 {
        return HttpResponse::BadRequest().body("bad request".to_string());
    }
    let value: i32 = p.1.parse().unwrap();
    match logic.update_counter(key, value) {
        Ok(value) => HttpResponse::Ok().body(value),
        Err(err) => HttpResponse::NotFound().body(err),
    }
}