use actix_web::{web, App, HttpServer, HttpRequest, Responder};
use dotenv::dotenv;

mod config;

const BASE_URL: &str = "https://euw1.api.riotgames.com";
const API_KEY: &str = "";

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("µPhenolphthalein");
    format!("Hello {}!", &name)
}

async fn get_summoner_by_name(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("yasuo xd");
    let summoner_url = format!("{}/lol/summoner/v4/summoners/by-name/{}?api_key={}", &BASE_URL, &name, &API_KEY);
    let body = reqwest::get(&summoner_url)
        .await.unwrap()
        .text()
        .await;
    format!("data: {:?}", &body)
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

    HttpServer::new(|| {
        App::new()
            .route("/riot/{name}", web::get().to(get_summoner_by_name))
            .route("/{name}", web::get().to(greet))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}