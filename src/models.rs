use super::schema::*;

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
