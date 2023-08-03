use std::{env, net::TcpListener};

use crate::adapters::{
    self,
    api::controllers::get_metric,
    api::app_state::AppState,
    spi::{
        http::{http_connection::HttpConnection},
    },
};
use actix_web::{dev::Server, middleware::Logger};
use actix_web::{web, App, HttpServer};
use crate::application::{
    usecases::usecase::UseCase,
    repositories::map::map_storage::Storage
};

pub fn server(listener: TcpListener, db_name: &str) -> Result<Server, std::io::Error> {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug");

    env_logger::try_init();

    let http_connection = HttpConnection {};
    let repo = &Storage::new();

    let static_reference: &'static Storage = unsafe { std::mem::transmute(Box::leak(Box::new(repo))) };
    let logic = UseCase::new(static_reference);
    let data = web::Data::new(AppState {
        app_name: String::from("Animal Facts API"),
        logic: logic,
    });

    let port = listener.local_addr().unwrap().port();

    let server = HttpServer::new(move || {
        App::new().app_data(data.clone()).service(get_metric)
    })
    .bind("127.0.0.1:8888")?
    .run();

    println!("Server running on port {}, db_name {}", port, db_name);

    Ok(server)
}
