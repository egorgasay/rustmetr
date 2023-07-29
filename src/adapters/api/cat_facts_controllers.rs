use crate::application::mappers::api_mapper::ApiMapper;
use crate::application::usecases::interfaces::AbstractUseCase;
use crate::{
    adapters::api::{
        cat_facts_mappers::CatFactPresenterMapper,
        cat_facts_presenters::CatFactPresenter,
        app_state::AppState,
        error_presenter::ErrorReponse,
    }
};


use actix_web::{get, web, HttpResponse};
use actix_web::body::BoxBody;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_cat_facts).service(get_one_random_cat_fact);
}

#[get("/")]
async fn get_all_cat_facts(data: web::Data<AppState<'_>>) -> Result<HttpResponse, ErrorReponse> {
    match data.logic.save("qwe".to_string(), 1i32).await {
        Some(..) => Ok(HttpResponse::Ok().body(BoxBody::new("ok"))),
        None => Ok(HttpResponse::Ok().body(BoxBody::new("error"))),
    }

}

#[get("/random")]
async fn get_one_random_cat_fact(data: web::Data<AppState<'_>>) -> Result<HttpResponse, ErrorReponse> {
    Ok(HttpResponse::Ok().into())
}
