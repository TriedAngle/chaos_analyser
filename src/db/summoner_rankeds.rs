#![allow(dead_code)]

use diesel::prelude::*;
use diesel::{PgConnection, QueryResult};

use crate::models::{NewSummonerRanked, SummonerRanked};

use crate::schema::summoner_rankeds;
use crate::schema::summoner_rankeds::dsl::summoner_rankeds as all_summoner_rankeds;

/// returning all SummonerRankeds
/// it is strongly advised to not use this function in production
/// as huge amount of data could exist
/// alternative is get_within_range_x
/// these alternative function return a range of values
/// but: they are not added yet:
/// TODO: ADD RANGE FUNCTIONS
pub async fn get_all(conn: &PgConnection) -> QueryResult<Vec<SummonerRanked>> {
    all_summoner_rankeds
        .order(summoner_rankeds::id.asc())
        .load::<SummonerRanked>(conn)
}

/// returning a SummonerRanked by its summoner_id as its id is only for database management
pub async fn get_by_summoner_id(summoner_id: i64, conn: &PgConnection) -> QueryResult<SummonerRanked> {
    all_summoner_rankeds
        .filter(summoner_rankeds::summoner_id.eq(summoner_id))
        .get_result::<SummonerRanked>(conn)
}

/// insert a NewSummonerRanked oro
/// updates a SummonerRanked
///
/// avoids duplicates (uses check_existing_summoner_ranked)
pub async fn insert_summoner_ranked(new_ranked: NewSummonerRanked, conn: &PgConnection) -> bool {
    if check_existing_summoner_ranked(new_ranked.summoner_id, conn).await {
        update_summoner_ranked(new_ranked, conn).await
    } else {
        diesel::insert_into(summoner_rankeds::table)
            .values(new_ranked)
            .get_result::<SummonerRanked>(conn)
            .is_ok()
    }
}

/// updates a SummonerRanked
/// calling this function is obsolete as it's automatically called by the insert function
/// if a SummonerRanked with the same summoner_id (its second unique identifier) exists
pub async fn update_summoner_ranked(new_ranked: NewSummonerRanked, conn: &PgConnection) -> bool {
    diesel::update(summoner_rankeds::table)
        .filter(summoner_rankeds::summoner_id.eq(new_ranked.summoner_id))
        .set(&new_ranked)
        .get_result::<SummonerRanked>(conn)
        .is_ok()
}

/// a helper function to check whether to update or insert a NewSummonerRanked / to avoid duplicates
/// using summoner_id instead of id as id is not known when inserting and summoner_id is also uinique
pub async fn check_existing_summoner_ranked(summoner_id: i64, conn: &PgConnection) -> bool {
    diesel::select(diesel::dsl::exists(
        summoner_rankeds::table.filter(summoner_rankeds::summoner_id.eq(summoner_id)),
    ))
    .get_result::<bool>(conn)
    .unwrap()
}
