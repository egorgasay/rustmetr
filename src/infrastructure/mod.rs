use std::{env, net::TcpListener};

use crate::adapters::{
    self,
    api::app_state::AppState,
    spi::{
        db::{db_connection::DbConnection},
        http::{http_repository::Storage, http_connection::HttpConnection},
    },
};
use actix_web::{dev::Server, middleware::Logger};
use actix_web::{web, App, HttpServer};
use crate::application::usecases::usecase::UseCase;
use crate::application::repositories::repository_abstract::RepositoryAbstract;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::RwLock;


pub fn server(listener: TcpListener, db_name: &str) -> Result<Server, std::io::Error> {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug");

    env_logger::try_init();

    let http_connection = HttpConnection {};
    let repo = &Storage::new();

    let static_reference: &'static Storage = unsafe { std::mem::transmute(Box::leak(Box::new(repo))) };
    let logic = UseCase::new(static_reference);
//    let mut data = web::Data::new(AppState {
//        app_name: String::from("Animal Facts API"),
//        logic: logic,
//    });

    let port = listener.local_addr().unwrap().port();

    let server = HttpServer::new(move || App::new().app_data(logic.clone()).wrap(Logger::default()).configure(adapters::api::routes::routes))
        .listen(listener)?
        .run();

    println!("Server running on port {}, db_name {}", port, db_name);

    Ok(server)
}
