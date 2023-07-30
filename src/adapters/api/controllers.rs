use crate::application::mappers::api_mapper::ApiMapper;
use crate::application::usecases::interfaces::AbstractUseCase;
use crate::{
    adapters::api::{
//        mappers::CatFactPresenterMapper,
//        presenters::CatFactPresenter,
        app_state::AppState,
        error_presenter::ErrorReponse,
    }
};
use std::sync::{Arc, Mutex};

use actix_web::{post, web, HttpResponse};
use actix_web::body::BoxBody;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(update_gauge).service(update_counter);
}


#[post("/update/gauge/{name}/{value}")]
async fn update_gauge(data: web::Data<Mutex<AppState<'_>>>, path: web::Path<(String,i32)>) -> Result<HttpResponse, ErrorReponse> {
    let p = path.into_inner();
    let mut name = p.0;
    let mut value = p.1;

    // println!("got {}: {} from /update/gauge", p.0, p.1);
    match data.lock().unwrap().logic.update_gauge(name, value) { //
        Some(..) =>
        {
            Ok(HttpResponse::Ok().body(BoxBody::new("ok")))
        },
        None => Ok(HttpResponse::Ok().body(BoxBody::new("error"))),
    }
}

#[post("/update/counter/{name}/{value}")]
async fn update_counter(data: web::Data<Mutex<AppState<'_>>>, path: web::Path<(String,String)>) -> Result<HttpResponse, ErrorReponse> {
    let p = path.into_inner();
    match data.lock().expect("msg").logic.update_counter(p.0, p.1) { //
        Some(..) => Ok(HttpResponse::Ok().body(BoxBody::new("ok"))),
        None => Ok(HttpResponse::Ok().body(BoxBody::new("error"))),
    }
}