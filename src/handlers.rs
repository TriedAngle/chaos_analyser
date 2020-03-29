use actix_web::{web, Responder, HttpRequest, client};
use crate::states::APIState;


pub async fn get_summoner_by_name(req: HttpRequest, data:  web::Data<APIState>) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("yasuo xd");
    let summoner_url = format!("{}/lol/summoner/v4/summoners/by-name/{}?api_key={}", &data.base_url, &name, &data.riot_api_key);
    let body = reqwest::get(&summoner_url)
        .await.unwrap()
        .text()
        .await;
    format!("data: {:?}", &body)
}