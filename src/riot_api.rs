use super::api_urls;
use super::config;
use super::models::Summoner;

pub async fn summoner_by_name(name: &str) -> Summoner {
    let summoner_url = api_urls::SUMMONER_URL_BY_NAME
        .replace("repx1", api_urls::BASE_URL_EUW)
        .replace("repx2", name)
        .replace("repx3", &config::get_riot_api_key());
    let summoner_data: &str = &reqwest::get(&summoner_url).await.unwrap().text().await.unwrap();
    Summoner::from_json(summoner_data)
}