use std::{env, net::TcpListener};

use crate::adapters::{
    api::controllers::get_metric,
    api::app_state::AppState,
};
use actix_web::{dev::Server};
use actix_web::{web, App, HttpServer};
use crate::application::{
    service::metric::MetricService,
    repositories::map::map_storage::Storage
};

pub fn server(listener: TcpListener, db_name: &str) -> Result<Server, std::io::Error> {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug");

    match env_logger::try_init(){
        Ok(_) => (),
        Err(e) => {
            panic!("{}", e);
        }
    };

    let repo = &Storage::default();

    let static_reference: &'static Storage = unsafe { std::mem::transmute(Box::leak(Box::new(repo))) };
    let logic = MetricService::new(static_reference);
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
