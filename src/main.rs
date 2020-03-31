#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;

use actix_web::{web, App, HttpServer};
use actix_web::{Responder, HttpRequest};
use dotenv::dotenv;

mod api_urls;
mod config;
mod db;
mod handlers;
mod models;
mod riot_api;
mod schema;
mod states;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();

    println!("---**--- Loaded Configurations ---**---");
    println!("ip: {}", config.server.host);
    println!("port: {}", config.server.port);
    println!("riot_api_key: {}", config::get_riot_api_key());
    println!("---**--- Server is starting    ---**---");

    HttpServer::new(move || {
        App::new()
        // .route("/", web::get().to(greet))
        // .route("/{name}", web::get().to(greet))
        .route("/riot/{name}",
           web::get().to(handlers::get_summoner_by_name),
       )
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
