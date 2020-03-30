#[derive(Queryable, Serialize, Deserialize)]
pub struct Summoner {
    account_id: String,
    profile_icon_id: i32,
    revision_date: i64,
    name: String,
    id: String,
    puuid: String,
    summoner_level: i64
}