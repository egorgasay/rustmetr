use rustmetric::adapters::{
    self,
    api::controllers::get_metric,
    api::app_state::AppState,
    spi::{
        http::{http_connection::HttpConnection},
    },
};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder,
    middleware::Logger};
use rustmetric::application::usecases::usecase::UseCase;
use rustmetric::application::repositories::map::map_storage::Storage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

   let storage = Storage::new();
   let static_reference: &'static Storage = unsafe { std::mem::transmute(Box::leak(Box::new(storage))) };
   let logic = web::Data::new(UseCase::new(static_reference));

   println!("started on 8080");
   HttpServer::new(move || {
       App::new().app_data(logic.clone())
           .wrap(Logger::default()).configure(adapters::api::routes::routes)
   })
   .bind("127.0.0.1:8080")?
   .run()
   .await
}