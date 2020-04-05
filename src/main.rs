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
    // load env file and extract data
    dotenv().ok();
    // new config, contains server host and port
    // (will probably be removed soon)
    let config: config::Config = config::Config::from_env().unwrap();
    // creates new connection manager and a pool for parallel database calls between multiple actors
    let connection_manager: ConnectionManager<PgConnection> =
        ConnectionManager::<PgConnection>::new(config::get_database_url());
    let pool: diesel::r2d2::Pool<ConnectionManager<PgConnection>> = r2d2::Pool::builder()
        .build(connection_manager)
        .expect("Failed to create pool!");

    // debug use only, display ip, port and riot api key
    println!("---**--- Loaded Configurations ---**---");
    println!("ip: {}", config.server.host);
    println!("port: {}", config.server.port);
    println!("riot_api_key: {}", config::get_riot_api_key());
    println!("---**--- Server is starting    ---**---");

    // creates new actix-web HttpServer with via factory
    // Tera ("user interface") and reqwest::Client ("http requests to the riot api") are shared within the whole application by adding it to the application data
    HttpServer::new(move || {
        let tera: Tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        let client = Client::new();
        App::new()
            .data(tera)
            .data(client)
            .data(pool.clone())
            .service(
                web::scope("/views")
                    .service(web::scope("/riot").route(
                        "/{region}/{summoner_name}",
                        web::get().to(handlers::views::riot_view::summoner_page),
                    ))
                    .service(web::scope("/db").route(
                        "{region}/{summoner_name}",
                        web::get().to(handlers::views::db_view::summoner_page),
                    )),
            )
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
