use actix_web::web::ServiceConfig;

pub mod rest;
pub mod riot;

pub fn endpoints(config: &mut ServiceConfig) {
    riot::endpoints(config);
    rest::endpoints(config)
}