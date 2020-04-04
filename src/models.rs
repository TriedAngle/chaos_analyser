use super::schema::*;
use serde_json::Value;
use std::convert::TryFrom;

const SOLO_QUEUE: &str = "RANKED_SOLO_5x5";

#[derive(Queryable, Serialize, Deserialize, Clone, Debug)]
pub struct Summoner {
    pub id: i64,
    pub puuid: String,
    pub account_id: String,
    pub profile_icon_id: i32,
    pub revision_date: i64,
    pub name: String,
    pub r_summoner_id: String,
    pub summoner_level: i64,
    pub region: String,
}

#[derive(Serialize, Insertable, Deserialize, Clone, Debug)]
#[table_name = "summoners"]
pub struct NewSummoner {
    pub puuid: String,
    pub account_id: String,
    pub profile_icon_id: i32,
    pub revision_date: i64,
    pub name: String,
    pub r_summoner_id: String,
    pub summoner_level: i64,
    pub region: String,
}

impl NewSummoner {
    pub fn from_json(data: &str, region: &str) -> NewSummoner {
        let v: Value = serde_json::from_str(data).unwrap();
        NewSummoner {
            puuid: v["puuid"].as_str().unwrap().to_string(),
            account_id: v["accountId"].as_str().unwrap().to_string(),
            profile_icon_id: v["profileIconId"].as_i64().unwrap() as i32,
            revision_date: v["revisionDate"].as_i64().unwrap(),
            name: v["name"].as_str().unwrap().to_string(),
            r_summoner_id: v["id"].as_str().unwrap().to_string(),
            summoner_level: v["summonerLevel"].as_i64().unwrap(),
            region: region.to_string(),
        }
    }
}

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct SummonerRanked {
    pub id: i64,
    pub summoner_id: i64,
    pub s_tier: String,
    pub f_tier: String,
    pub s_rank: String,
    pub f_rank: String,
    pub s_league_points: i32,
    pub f_league_points: i32,
    pub s_wins: i32,
    pub f_wins: i32,
    pub s_losses: i32,
    pub f_losses: i32,
    pub s_hot_streak: bool,
    pub f_hot_streak: bool,
    pub s_veteran: bool,
    pub f_veteran: bool,
    pub s_fresh_blood: bool,
    pub f_fresh_blood: bool,
    pub s_inactive: bool,
    pub f_inactive: bool,
    pub s_is_ms: bool,
    pub f_is_ms: bool,
    pub s_ms_w: i32,
    pub f_ms_w: i32,
    pub s_ms_l: i32,
    pub f_ms_l: i32,
    pub s_ms_prg: String,
    pub f_ms_prg: String,
    pub s_ms_trg: i32,
    pub f_ms_trg: i32,
}

#[derive(Serialize, Insertable, Deserialize, AsChangeset, Clone, Debug)]
#[table_name = "summoner_rankeds"]
pub struct NewSummonerRanked {
    pub summoner_id: i64,
    pub s_tier: String,
    pub f_tier: String,
    pub s_rank: String,
    pub f_rank: String,
    pub s_league_points: i32,
    pub f_league_points: i32,
    pub s_wins: i32,
    pub f_wins: i32,
    pub s_losses: i32,
    pub f_losses: i32,
    pub s_hot_streak: bool,
    pub f_hot_streak: bool,
    pub s_veteran: bool,
    pub f_veteran: bool,
    pub s_fresh_blood: bool,
    pub f_fresh_blood: bool,
    pub s_inactive: bool,
    pub f_inactive: bool,
    pub s_is_ms: bool,
    pub f_is_ms: bool,
    pub s_ms_w: i32,
    pub f_ms_w: i32,
    pub s_ms_l: i32,
    pub f_ms_l: i32,
    pub s_ms_prg: String,
    pub f_ms_prg: String,
    pub s_ms_trg: i32,
    pub f_ms_trg: i32,
}

impl NewSummonerRanked {
    pub fn from_json(data: &str, summoner_id: i64) -> NewSummonerRanked {
        let v: Value = serde_json::from_str(data).unwrap();

        let x: usize;
        let y: usize;

        if v[0]["queueType"].as_str().unwrap() == SOLO_QUEUE {
            x = 0;
            y = 1;
        } else {
            x = 1;
            y = 0;
        }

        let mut s_tier: String = String::from("");
        let mut f_tier: String = String::from("");
        let mut s_rank: String = String::from("");
        let mut f_rank: String = String::from("");
        let mut s_league_points: i32 = 0;
        let mut f_league_points: i32 = 0;
        let mut s_wins: i32 = 0;
        let mut f_wins: i32 = 0;
        let mut s_losses: i32 = 0;
        let mut f_losses: i32 = 0;
        let mut s_hot_streak: bool = false;
        let mut f_hot_streak: bool = false;
        let mut s_veteran: bool = false;
        let mut f_veteran: bool = false;
        let mut s_fresh_blood: bool = false;
        let mut f_fresh_blood: bool = false;
        let mut s_inactive: bool = false;
        let mut f_inactive: bool = false;

        let mut s_is_ms: bool = false;
        let mut f_is_ms: bool = false;
        let mut s_ms_w: i32 = 0;
        let mut f_ms_w: i32 = 0;
        let mut s_ms_l: i32 = 0;
        let mut f_ms_l: i32 = 0;
        let mut s_ms_prg: String = String::from("");
        let mut f_ms_prg: String = String::from("");
        let mut s_ms_trg: i32 = 0;
        let mut f_ms_trg: i32 = 0;

        if !v[x]["tier"].is_null() {
            s_tier = v[x]["tier"].as_str().unwrap().to_string();
            s_rank = v[x]["rank"].as_str().unwrap().to_string();
            s_league_points = v[x]["leaguePoints"].as_i64().unwrap() as i32;
            s_wins = v[x]["wins"].as_i64().unwrap() as i32;
            s_losses = v[x]["losses"].as_i64().unwrap() as i32;
            s_hot_streak = v[x]["hotStreak"].as_bool().unwrap();
            s_veteran = v[x]["veteran"].as_bool().unwrap();
            s_fresh_blood = v[x]["freshBlood"].as_bool().unwrap();
            s_inactive = v[x]["inactive"].as_bool().unwrap();

            if !v[x]["miniSeries"].is_null() {
                s_is_ms = true;
                s_ms_w = v[x]["miniSeries"]["wins"].as_i64().unwrap() as i32;
                s_ms_l = v[x]["miniSeries"]["losses"].as_i64().unwrap() as i32;
                s_ms_prg = v[x]["miniSeries"]["progress"].as_str().unwrap().to_string();
                s_ms_trg = v[x]["miniSeries"]["target"].as_i64().unwrap() as i32;
            }
        }

        if !v[y]["tier"].is_null() {
            f_tier = v[y]["tier"].as_str().unwrap().to_string();
            f_rank = v[y]["rank"].as_str().unwrap().to_string();
            f_league_points = v[y]["leaguePoints"].as_i64().unwrap() as i32;
            f_wins = v[y]["wins"].as_i64().unwrap() as i32;
            f_losses = v[y]["losses"].as_i64().unwrap() as i32;  
            f_hot_streak = v[y]["hotStreak"].as_bool().unwrap();
            f_veteran = v[y]["veteran"].as_bool().unwrap();
            f_fresh_blood = v[y]["freshBlood"].as_bool().unwrap();
            f_inactive = v[y]["inactive"].as_bool().unwrap();

            if !v[y]["miniSeries"].is_null() {
                f_is_ms = true;
                f_ms_w = v[y]["miniSeries"]["wins"].as_i64().unwrap() as i32;
                f_ms_l = v[y]["miniSeries"]["losses"].as_i64().unwrap() as i32;
                f_ms_prg = v[y]["miniSeries"]["progress"].as_str().unwrap().to_string();
                f_ms_trg = v[y]["miniSeries"]["target"].as_i64().unwrap() as i32;
            }

        }

        NewSummonerRanked {
            summoner_id,
            s_tier,
            f_tier,
            s_rank,
            f_rank,
            s_league_points,
            f_league_points,
            s_wins,
            f_wins,
            s_losses,
            f_losses,
            s_hot_streak,
            f_hot_streak,
            s_veteran,
            f_veteran,
            s_fresh_blood,
            f_fresh_blood,
            s_inactive,
            f_inactive,
            s_is_ms,
            f_is_ms,
            s_ms_w,
            f_ms_w,
            s_ms_l,
            f_ms_l,
            s_ms_prg,
            f_ms_prg,
            s_ms_trg,
            f_ms_trg,
        }
    }

    pub fn set_summoner_id(mut self, summoner_id: i64){
        self.summoner_id = summoner_id;
    }
}
