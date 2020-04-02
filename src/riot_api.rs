use super::api_urls;
use super::config;
use super::models::Summoner;

use reqwest::Client;

pub async fn summoner_by_name(name: &str, region: &str, client: &Client) -> Summoner {
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

    Summoner::from_json(summoner_data, &region)
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
