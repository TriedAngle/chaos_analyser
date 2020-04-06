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
            .service(
                web::scope("/api")
                .service(
                    web::scope("/db")
                    .service(
                        web::scope("/summoners")
                            .route("/", web::get().to(handlers::api::db_api::get_summoner_all))
                            .route("/{id}", web::get().to(handlers::api::db_api::get_summoner_by_id))
                            .route("/by-name/{summoner_name}", web::get().to(handlers::api::db_api::get_summoners_by_name))
                            .route("/by-puuid/{puuid}", web::get().to(handlers::api::db_api::get_summoner_by_puuid))
                            .route("/by-region-name/{region}/{summoner_name}", web::get().to(handlers::api::db_api::get_summoner_by_name_and_region))
                            .route("/add-by-region-name/{region}/{summoner_name}", web::get().to(handlers::api::db_api::add_summoner_by_name_and_region))
                    )
                    .service(
                        web::scope("summoner-rankeds")
                            .route("/", web::get().to(handlers::api::db_api::get_summoner_rankeds_all))
                            .route("/{id}", web::get().to(handlers::api::db_api::get_summoner_ranked_by_summoner_id))
                            .route("/add-by-s-id/{summoner_id}", web::get().to(handlers::api::db_api::add_summoner_ranked_by_summoner_id))
                    )
                )
                .service(
                    web::scope("/riot")
                    .service(
                        web::scope("/summoners/")
                            .route("/by-region-name/{region}/{name}", web::get().to(handlers::api::riot_api::get_summoner_by_name_and_region))
                            .route("/by-puuid/{region}/{puuid}", web::get().to(handlers::api::riot_api::get_summoner_by_puuid_and_region))
                    )
                    .service(
                        web::scope("summoner-rankeds")
                            .route("/by-region-r-id/{region}/{r_summoner_id}", web::get().to(handlers::api::riot_api::get_summoner_riot_summoner_ranked_by_id_and_region))
                            .route("/with-data-id/{region}/{summoner_id}/{r_summoner_id}", web::get().to(handlers::api::riot_api::get_summoner_by_name_and_region))
                    )
                )
            )
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
