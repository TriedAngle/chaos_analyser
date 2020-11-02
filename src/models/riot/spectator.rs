use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CurrentGameInfo {
    pub game_id: i64,
    pub game_type: String,
    pub game_start_time: i64,
    pub map_id: i64,
    pub game_length: i64,
    pub platform_id: String,
    pub game_mode: String,
    pub banned_champions: Vec<BannedChampion>,
    pub game_queue_config_id: i64,
    pub observers: Observer,
    pub participants: Vec<CurrentGameParticipant>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BannedChampion {
    pub pick_turn: i32,
    pub champion_id: i64,
    pub team_id: i64,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Observer {
    pub encryption_key: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CurrentGameParticipant {
    pub champion_id: i64,
    pub perks: Perks,
    pub profile_icon_id: i64,
    pub bot: bool,
    pub team_id: i64,
    pub summoner_name: String,
    pub spell1_id: i64,
    pub spell2_id: i64,
    pub game_customization_objects: Vec<GameCustomizationObject>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Perks {
    pub perk_ids: Vec<i64>,
    pub perk_style: i64,
    pub perk_sub_style: i64,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameCustomizationObject {
    pub category: String,
    pub content: String,
}
