use std::{env, net::TcpListener};

use crate::adapters::{
    self,
    api::app_state::AppState,
    spi::{
        db::{db_connection::DbConnection},
        http::{http_cat_facts_repository::CatFactsRepository, http_connection::HttpConnection},
    },
};
use actix_web::{dev::Server, middleware::Logger};
use actix_web::{web, App, HttpServer};
use crate:: application::usecases::{get_all_cat_facts_usecase::UseCase, get_one_random_cat_fact_usecase::GetOneRandomCatFactUseCase};


pub fn server(listener: TcpListener, db_name: &str) -> Result<Server, std::io::Error> {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug");

    env_logger::try_init();

    let db_connection = DbConnection { db_name: db_name.to_string() };
    let http_connection = HttpConnection {};
    let repo = &CatFactsRepository {
        http_connection,
        source: dotenv::var("CATS_SOURCE").expect("CATS_SOURCE must be set"),
    };

    let static_reference: &'static CatFactsRepository = unsafe { std::mem::transmute(Box::leak(Box::new(repo))) };

    let data = web::Data::new(AppState {
        app_name: String::from("Animal Facts API"),
        logic: UseCase::new(static_reference),
    });

    let port = listener.local_addr().unwrap().port();

    let server = HttpServer::new(move || App::new().app_data(data.clone()).wrap(Logger::default()).configure(adapters::api::routes::routes))
        .listen(listener)?
        .run();

    println!("Server running on port {}, db_name {}", port, db_name);

    Ok(server)
}
