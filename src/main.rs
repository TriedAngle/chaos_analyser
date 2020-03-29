use actix_web::{web, App, HttpServer, HttpRequest, Responder};
use dotenv::dotenv;
use std::sync::Mutex;

mod config;
mod handlers;
mod states;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("µPhenolphthalein");
    format!("Hello {}!", &name)
}



#[actix_rt::main]
async fn main() -> std::io::Result<()> {    
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();

    println!("---**--- Loaded Configurations ---**---");
    println!("ip: {}", config.server.host);
    println!("port: {}", config.server.port);
    println!("riot_api_key: {}", config.api_key);
    println!("---**--- Server is starting    ---**---");
    
    let api_state = states::APIState{
        riot_api_key: config.api_key.clone(),
        base_url: config.base_url.clone(),
    };

    HttpServer::new(move || {
        App::new()
            .data(api_state.clone())
            .route("/riot/{name}", web::get().to(handlers::get_summoner_by_name))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}