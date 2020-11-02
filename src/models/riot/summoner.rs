use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SummonerDTO {
    pub id: String,
    pub account_id: String,
    pub puuid: String,
    pub profile_icon_id: i32,
    pub revision_date: i64,
    pub name: String,
    pub summoner_level: i64,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountDTO {
    pub puuid: String,
    pub game_name: String,
    pub tag_line: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActiveShardDTO {
    pub puuid: String,
    pub game: String,
    pub active_shard: String,
}
