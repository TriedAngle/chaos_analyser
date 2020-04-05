#![allow(dead_code)]

use super::api_urls;
use super::config;
use super::models::{NewSummoner, NewSummonerRanked, Summoner};

use reqwest::Client;

pub async fn summoner_by_name(name: &str, region: &str, client: &Client) -> NewSummoner {
    let region_link = get_region_link(&region);
    let summoner_url = api_urls::SUMMONER_URL_BY_NAME
        .replace("in1", region_link)
        .replace("in2", name)
        .replace("in3", &config::get_riot_api_key());

    let summoner_data: &str = &client
        .get(&summoner_url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    NewSummoner::from_json(summoner_data, &region)
}

pub async fn summoner_ranked_by_id(
    r_summoner_id: &str,
    summoner_id: i64,
    region: &str,
    client: &Client,
) -> NewSummonerRanked {
    let region_link = get_region_link(&region);
    let summoner_ranked_url = api_urls::SUMMONER_RANK_URL_BY_SUMMONER_ID
        .replace("in1", region_link)
        .replace("in2", r_summoner_id)
        .replace("in3", &config::get_riot_api_key());

    let summoner_ranked_data: &str = &client
        .get(&summoner_ranked_url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    NewSummonerRanked::from_json(summoner_ranked_data, summoner_id)
}

pub async fn get_summoner_id_from_new_summoner(new_summoner: &NewSummoner) -> &str {
    new_summoner.r_summoner_id.as_str()
}

pub async fn get_summoner_id_from_summoner(summoner: &Summoner) -> &str {
    summoner.r_summoner_id.as_str()
}

fn get_region_link(region: &str) -> &str {
    match region {
        "BR" => api_urls::BASE_URL_BR,
        "EUNE" => api_urls::BASE_URL_EUNE,
        "EUW" => api_urls::BASE_URL_EUW,
        "JP" => api_urls::BASE_URL_JP,
        "KR" => api_urls::BASE_URL_KR,
        "LAN" => api_urls::BASE_URL_LAN,
        "LAS" => api_urls::BASE_URL_LAS,
        "NA" => api_urls::BASE_URL_NA,
        "OCE" => api_urls::BASE_URL_OCE,
        "TR" => api_urls::BASE_URL_TR,
        "RU" => api_urls::BASE_URL_RU,
        &_ => api_urls::BASE_URL_EUW,
    }
}
