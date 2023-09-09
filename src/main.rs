use rustmetric::adapters::{
    self,
};

use actix_web::{web, App, HttpServer,
    middleware::Logger};
use tera::Tera;
use rustmetric::application::service::metric::MetricService;
use rustmetric::application::repositories::map::map_storage::Storage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let storage = Storage::default();
   let static_reference: &'static Storage = unsafe { std::mem::transmute(Box::leak(Box::new(storage))) };
   let logic = web::Data::new(MetricService::new(static_reference));

   println!("started on 8080");
   HttpServer::new(move || {
       let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
       App::new().
           app_data(web::Data::new(tera)).
           app_data(logic.clone())
           .wrap(Logger::default()).configure(adapters::api::routes::routes)
   })
   .bind("127.0.0.1:8080")?
   .run()
   .await
}