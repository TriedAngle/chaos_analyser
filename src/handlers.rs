use actix_web::{web, Responder, HttpRequest};
use crate::config::Config;


pub async fn get_summoner_by_name(req: HttpRequest, config:  web::Data<Config>) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("yasuo xd");
    let summoner_url = format!("{}/lol/summoner/v4/summoners/by-name/{}?api_key={}", &config.base_url, &name, &config.api_key);
    let body = reqwest::get(&summoner_url)
        .await.unwrap()
        .text()
        .await;
    format!("data: {:?}", &body)
}