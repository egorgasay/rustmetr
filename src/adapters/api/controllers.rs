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
        Ok(value) => HttpResponse::Ok().body(value.to_string()),
        Err(err) => HttpResponse::NotFound().body(err),
    }
}

#[post("/update/gauge/{key}/{value}")]
async fn update_gauge(logic: web::Data<UseCase<'_>>, path: web::Path<(String,String)>) -> impl Responder {
    let p = path.into_inner();
    let key = p.0;
    let mut value: f32 = 0 as f32;
    match p.1.parse::<f32>() {
        Ok(n) => value = n,
        Err(e) => {
            return HttpResponse::BadRequest().body("bad request".to_string());
        },
    }

    match logic.update_gauge(key, value) {
        Ok(..) => HttpResponse::Ok().body("completed successfully"),
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

    let mut value: i32 = 0;
    match p.1.parse::<i32>() {
        Ok(n) => value = n,
        Err(e) => {
            return HttpResponse::BadRequest().body("bad request".to_string());
        },
    }

    match logic.update_counter(key, value) {
        Ok(value) => HttpResponse::Ok().body("completed successfully"),
        Err(err) => HttpResponse::NotFound().body(err),
    }
}