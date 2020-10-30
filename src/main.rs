use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use log::info;
use std::sync::Arc;
use chaotic_analyzer::{setup_logger, endpoints};
use chaotic_analyzer::config::Config;
use chaotic_analyzer::db::{PgPool, new_pool};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Delete latest logging file
    // Disable this in production!
    // TODO: move latest log file into a logfile directory with timestamps.
    match std::fs::read("output.log") {
        Ok(_) => std::fs::remove_file("output.log").unwrap(),
        Err(_) => println!("no log file"),
    };

    match setup_logger() {
        Ok(()) => {}
        Err(err) => println!("Error: {}, while configuring logger", err),
    };

    // Setup Config
    let config: Arc<Config> = Arc::new(Config::new());
    let server_address = config.server_address.clone();
    // log the config
    info!("Starting Server with following configuration \n {}", config);

    let pool: PgPool = new_pool(&config);

    // Edit Cors for production
    HttpServer::new(move || {
        let cors = Cors::default()
            .supports_credentials()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .data(config.clone())
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            // .configure(endpoints::graphql::endpoints)
            .configure(endpoints::rest::endpoints)
    })
        .bind(server_address)?
        .run()
        .await
}
