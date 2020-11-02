use crate::models::riot::summoner::SummonerDTO;
use crate::riot::api::{KEY, SUMMONER_BY_ID, SUMMONER_BY_NAME};
use crate::types::region::Region;
use reqwest::Client;

pub async fn summoner_by_id(
    id: String,
    region: Region,
    client: &Client,
    key: &String,
) -> Result<SummonerDTO, reqwest::Error> {
    let url = format!("{}{}{}", region.as_str(), SUMMONER_BY_ID, id);
    let data = client
        .get(&url)
        .header(KEY, key)
        .send()
        .await?
        .text()
        .await?;
    println!("data: {:?}", data);
    let summoner: SummonerDTO = serde_json::from_str(&data).unwrap();
    Ok(summoner)
}

pub async fn summoner_by_name(
    name: String,
    region: Region,
    client: &Client,
    key: &String,
) -> Result<SummonerDTO, reqwest::Error> {
    let url = format!("{}{}{}", region.as_str(), SUMMONER_BY_NAME, name);
    let data = client
        .get(&url)
        .header(KEY, key)
        .send()
        .await?
        .text()
        .await?;
    let summoner: SummonerDTO = serde_json::from_str(&data).unwrap();
    Ok(summoner)
}
