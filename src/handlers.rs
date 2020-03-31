use actix_web::{HttpRequest, Responder};
use super::riot_api;
use super::models::Summoner;

pub async fn get_summoner_by_name(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("yasuo xd");
    let summoner_data: Summoner = riot_api::summoner_by_name(&name).await;
    format!("summoner name: {} | {}", &summoner_data.name, &summoner_data.summoner_level)
}
