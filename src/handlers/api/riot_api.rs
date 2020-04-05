use actix_web::{web, HttpRequest, HttpResponse};
use reqwest::Client;

use crate::riot_api;

// ###### SUMMONER ######

/// Returns a single Summoner as the name is unique for each region
pub async fn get_summoner_by_name_and_region(req: HttpRequest, client: web::Data<Client>) -> HttpResponse {
    let summoner_name: &str = req.match_info()
        .get("summoner_name")
        .unwrap_or("µPhenolphthalein");

    let summoner_region: &str = req.match_info()
        .get("region")
        .unwrap_or("EUW");

    HttpResponse::Ok().json(riot_api::summoner_by_name(summoner_name, summoner_region, &client).await).await.unwrap()
}

/// Returns a single SummonerRanked (with database id)
pub async fn get_summoner_ranked_by_ids_and_region(req: HttpRequest, client: web::Data<Client>) -> HttpResponse {
    let r_summoner_id: &str = req.match_info()
        .get("r_summoner_id")
        .unwrap();

    let summoner_id: i64 = req.match_info()
        .get("summoner_id")
        .unwrap_or("1")
        .parse().unwrap_or(1);

    let region: &str = req.match_info()
        .get("region")
        .unwrap();


    HttpResponse::Ok().json(riot_api::summoner_ranked_by_id(r_summoner_id, summoner_id, region, &client).await).await.unwrap()
}

/// Returns a single RiotSummonerRanked (without database id)
pub async fn get_summoner_riot_summoner_ranked_by_id_and_region(req: HttpRequest, client: web::Data<Client>) -> HttpResponse {
    let r_summoner_id: &str = req.match_info()
        .get("r_summoner_id")
        .unwrap();

    let summoner_region: &str = req.match_info()
        .get("region")
        .unwrap_or("EUW");

    HttpResponse::Ok().json(riot_api::riot_summoner_ranked_by_r_summoner_id(r_summoner_id, summoner_region, &client).await).await.unwrap()
}