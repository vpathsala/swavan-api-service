pub mod post;
pub mod get;
pub mod delete;
pub mod put;
pub mod common;

use actix_web::web;

use crate::config;

pub fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .data( config::Configuration{ ..Default::default()} )
            .service(web::resource("/{id}").route(web::get().to(get::get)))
            .service(web::resource("/")
                .route(web::delete().to(delete::delete))
                .route(web::post().to(post::post))
                .route(web::put().to(put::put)))
    );
}