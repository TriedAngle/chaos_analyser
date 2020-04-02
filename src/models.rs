use super::schema::*;
use serde_json::Value;

#[derive(Queryable, Serialize, Insertable, Deserialize, Clone)]
#[table_name = "summoners"]
pub struct Summoner {
    pub account_id: String,
    pub profile_icon_id: i32,
    pub revision_date: i64,
    pub name: String,
    pub summoner_id: String,
    pub puuid: String,
    pub summoner_level: i64,
}

impl Summoner {
    pub fn from_json(data: &str) -> Summoner {
        let v: Value = serde_json::from_str(data).unwrap();
        Summoner {
            account_id: v["accountId"].as_str().unwrap().to_string(),
            profile_icon_id: v["profileIconId"].as_i64().unwrap() as i32,
            revision_date: v["revisionDate"].as_i64().unwrap(),
            name: v["name"].as_str().unwrap().to_string(),
            summoner_id: v["id"].as_str().unwrap().to_string(),
            puuid: v["puuid"].as_str().unwrap().to_string(),
            summoner_level: v["summonerLevel"].as_i64().unwrap(),
        }
    }
}

pub struct LeagueEntry {
    pub puuid: String,
    pub league_id: String,
    pub summoner_id: String,
    pub summoner_name: String,
    pub queue_type: String,
    pub tier: String,
    pub rank: String,
    pub league_points: i32,
    pub wins: i32,
    pub losses: i32,
    pub hot_streak: bool,
    pub veteran: bool,
    pub fresh_blood: bool,
    pub inactive: bool,
    pub mini_series: MiniSeries,
}

#[allow(dead_code)]
impl LeagueEntry {
    pub fn from_json(data: &str) -> LeagueEntry {
        let v: Value = serde_json::from_str(data).unwrap();
        LeagueEntry {
            puuid: v["puuid"].as_str().unwrap().to_string(),
            league_id: v["leagueId"].as_str().unwrap().to_string(),
            summoner_id: v["summonerId"].as_str().unwrap().to_string(),
            summoner_name: v["summonerName"].as_str().unwrap().to_string(),
            queue_type: v["id"].as_str().unwrap().to_string(),
            tier: v["puuid"].as_str().unwrap().to_string(),
            rank: v["summonerLevel"].as_str().unwrap().to_string(),
            league_points: v["leaguePoints"].as_i64().unwrap() as i32,
            wins: v["wins"].as_i64().unwrap() as i32,
            losses: v["losses"].as_i64().unwrap() as i32,
            hot_streak: v["hotStreak"].as_bool().unwrap(),
            veteran: v["veteran"].as_bool().unwrap(),
            fresh_blood: v["freshBlood"].as_bool().unwrap(),
            inactive: v["inactive"].as_bool().unwrap(),
            mini_series: MiniSeries::from_value(&v),
        }
    }
}

#[allow(dead_code)]
pub struct MiniSeries {
    pub losses: i32,
    pub progress: String,
    pub target: i32,
    pub wins: i32,
}

#[allow(dead_code)]
impl MiniSeries {
    pub fn from_json(data: &str) -> MiniSeries {
        let v: Value = serde_json::from_str(data).unwrap();
        MiniSeries {
            losses: v["losses"].as_i64().unwrap() as i32,
            progress: v["progress"].as_str().unwrap().to_string(),
            target: v["target"].as_i64().unwrap() as i32,
            wins: v["wins"].as_i64().unwrap() as i32,
        }
    }

    pub fn from_value(v: &Value) -> MiniSeries {
        MiniSeries {
            losses: v["miniSeries.losses"].as_i64().unwrap() as i32,
            progress: v["miniSeries.progress"].as_str().unwrap().to_string(),
            target: v["miniSeries.progress"].as_i64().unwrap() as i32,
            wins: v["miniSeries.progress"].as_i64().unwrap() as i32,
        }
    }
}
