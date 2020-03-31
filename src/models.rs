use super::schema::*;
use serde_json::{Value};

#[derive(Queryable, Serialize)]
pub struct Summoner {
    pub account_id: String,
    pub profile_icon_id: i32,
    pub revision_date: i64,
    pub name: String,
    pub id: String,
    pub puuid: String,
    pub summoner_level: i64,
}

#[derive(Insertable, Deserialize)]
#[table_name = "summoners"]
pub struct NewSummoner {
    pub account_id: String,
    pub profile_icon_id: i32,
    pub revision_date: i64,
    pub name: String,
    pub id: String,
    pub puuid: String,
    pub summoner_level: i64,
}

impl Summoner {
    pub async fn from_json(data: &str) -> Summoner {
        let v: Value = serde_json::from_str(data).unwrap();
        Summoner {
            account_id: v["accountId"].as_str().unwrap().to_string(),
            profile_icon_id: v["profileIconId"].as_i64().unwrap() as i32,
            revision_date: v["revisionDate"].as_i64().unwrap(),
            name: v["name"].as_str().unwrap().to_string(),
            id: v["id"].as_str().unwrap().to_string(),
            puuid: v["puuid"].as_str().unwrap().to_string(),
            summoner_level: v["summonerLevel"].as_i64().unwrap()
        }
    }
}