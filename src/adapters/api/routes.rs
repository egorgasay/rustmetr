use actix_web::web;

use crate::adapters::api::cat_facts_controllers;

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("/api/v1/cats").configure(cat_facts_controllers::routes));
}
