use actix_web::web;

use crate::adapters::api::controllers;

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("").
            configure(controllers::routes));
}
