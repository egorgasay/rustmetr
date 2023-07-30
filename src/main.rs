use animal_facts_api::adapters::{
    self,
    api::controllers::get_metric,
    api::app_state::AppState,
    spi::{
        db::{db_connection::DbConnection},
        http::{http_repository::Storage, http_connection::HttpConnection},
    },
};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder,
    middleware::Logger};
use std::collections::HashMap;
use std::sync::Mutex;
use animal_facts_api::application::usecases::usecase::UseCase;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

   let storage = Storage::new();
   let static_reference: &'static Storage = unsafe { std::mem::transmute(Box::leak(Box::new(storage))) };
   let logic = web::Data::new(UseCase::new(static_reference));

   println!("started on 6789");
   HttpServer::new(move || {
       App::new().app_data(logic.clone())
           .wrap(Logger::default()).configure(adapters::api::routes::routes)
   })
   .bind("127.0.0.1:6789")?
   .run()
   .await
}