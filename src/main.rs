
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;

use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use reqwest::Client;
use tera::Tera;

mod api_urls;
mod config;
mod db;
mod handlers;
mod models;
mod riot_api;
mod schema;
mod states;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = config::Config::from_env().unwrap();
    let connection_manager = ConnectionManager::<PgConnection>::new(config::get_database_url());
    let pool = r2d2::Pool::builder()
        .build(connection_manager)
        .expect("Failed to create pool!");

    println!("---**--- Loaded Configurations ---**---");
    println!("ip: {}", config.server.host);
    println!("port: {}", config.server.port);
    println!("riot_api_key: {}", config::get_riot_api_key());
    println!("---**--- Server is starting    ---**---");

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        let client = Client::new();
        App::new().data(tera).data(client).data(pool.clone()).route(
            "/rito/{region}/{summoner_name}",
            web::get().to(handlers::summoner_page),
        )
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}