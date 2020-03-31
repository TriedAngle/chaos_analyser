use actix_web::{HttpRequest, Responder};
use super::riot_api;

pub async fn get_summoner_by_name(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("yasuo xd");
    let summoner_data = riot_api::summoner_by_name(&name);
    format!("data: {}", &summoner_data.await.unwrap())
}
