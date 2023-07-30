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
use std::sync::{Mutex, RwLock};

use actix_web::{post, web, HttpResponse};
use actix_web::body::BoxBody;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(update_gauge); //.service(update_counter);
}


#[post("/update/gauge/{name}/{value}")]
async fn update_gauge(data: web::Data<Mutex<AppState<'_>>>, path: web::Path<(String,i32)>) -> Result<HttpResponse, ErrorReponse> {
    let p = path.into_inner();
    let mut name = p.0;
    let mut value = p.1;
    // println!("got {}: {} from /update/gauge", p.0, p.1);
//    match data.lock().expect("Failed to lock data").logic.update_gauge(name, value) { //
//        Some(..) =>
//        {
//            Ok(HttpResponse::Ok().body(BoxBody::new("ok")))
//        },
//        None => Ok(HttpResponse::Ok().body(BoxBody::new("error"))),
//    }

    let app_state = data.lock().unwrap();
    let mut my_data = app_state.logic.lock().unwrap(); // Access the mutable data

    my_data.update_gauge(name, value);
    // Perform your modifications on my_data here
    // For example:
    // my_data.modify(path.0, path.1);

    // Return a response (replace with your own logic)
    Ok(HttpResponse::Ok().finish())
}

//#[post("/update/counter/{name}/{value}")]
//async fn update_counter(data: web::Data<RwLock<AppState<'_>>>, path: web::Path<(String,String)>) -> Result<HttpResponse, ErrorReponse> {
//    let p = path.into_inner();
////    match data.lock().expect("msg").logic.update_counter(p.0, p.1) { //
////        Some(..) => Ok(HttpResponse::Ok().body(BoxBody::new("ok"))),
////        None => Ok(HttpResponse::Ok().body(BoxBody::new("error"))),
////    }
//
//    match data.write() {
//    Ok(mut lock) => {
//        lock.logic.update_counter(p.0, p.1);
//        Ok(HttpResponse::Ok().body("ok"))
//    },
//        Err(_) => Ok(HttpResponse::InternalServerError().body("error")),
//    }
//}