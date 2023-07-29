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


use actix_web::{post, web, HttpResponse};
use actix_web::body::BoxBody;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(update_gauge);
}


#[post("/update/gauge/{name}/{value}")]
async fn update_gauge(data: web::Data<AppState<'_>>, path: web::Path<(String,i32)>) -> Result<HttpResponse, ErrorReponse> {
    let p = path.into_inner();
    println!("got {}: {} from /update/gauge", p.0, p.1);
    match data.logic.save(p.0, p.1).await { //
        Some(..) => Ok(HttpResponse::Ok().body(BoxBody::new("ok"))),
        None => Ok(HttpResponse::Ok().body(BoxBody::new("error"))),
    }
}
