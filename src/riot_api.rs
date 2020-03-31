use super::api_urls;
use super::config;
use super::models::Summoner;

pub async fn summoner_by_name(name: &str) {
    let summoner_url = api_urls::SUMMONER_URL_BY_NAME
        .replace("repx1", api_urls::BASE_URL_EUW)
        .replace("repx2", name)
        .replace("repx3", &config::get_riot_api_key());

    let body = reqwest::get(&summoner_url).await.unwrap().text().await;
    println!("{:?}", body);
}
