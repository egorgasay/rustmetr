use std::env;
use std::net::TcpListener;

use animal_facts_api::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let environment_file;
    if let Ok(e) = env::var("ENV") {
        environment_file = format!(".env.{}", e);
    } else {
        environment_file = String::from(".env");
    }

    dotenv::from_filename(environment_file).ok();

    let listener = TcpListener::bind("0.0.0.0:8888").expect("Failed to bind random port");
    let database_name = dotenv::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

    run(listener, &database_name)?.await
}

//
//use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
//use std::collections::HashMap;
//use std::sync::Mutex;
//
//#[actix_web::main]
//async fn main() -> std::io::Result<()> {
//    let storage = Storage::new();
//    let logic = web::Data::new(UseCase::new(storage));
//
//    HttpServer::new(move || {
//        App::new()
//            .app_data(logic.clone())
//            .service(get_metric)
//            .service(update_gauge)
//            .service(update_counter)
//    })
//    .bind("127.0.0.1:8888")?
//    .run()
//    .await
//}