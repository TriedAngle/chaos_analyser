use diesel::prelude::*;
use diesel::PgConnection;
use diesel::QueryResult;

use super::models::{NewSummoner, Summoner, NewSummonerRanked, SummonerRanked};

use super::schema::summoners;
use super::schema::summoners::dsl::summoners as all_summoners;

use super::schema::summoner_rankeds;
use super::schema::summoner_rankeds::dsl::summoner_rankeds as all_summoner_rankeds;

pub fn get_all(conn: &PgConnection) -> QueryResult<Vec<Summoner>> {
    all_summoners
        .order(summoners::id.asc())
        .load::<Summoner>(conn)
}

pub fn get_by_id(id: i64, conn: &PgConnection) -> QueryResult<Summoner> {
    all_summoners
        .find(summoners::id)
        .get_result::<Summoner>(conn)
}

pub fn get_by_puuid(puuid: &str, conn: &PgConnection) -> QueryResult<Summoner> {
    all_summoners
        .filter(summoners::puuid.eq(puuid))
        .get_result::<Summoner>(conn)
}

pub fn insert_summoner(new_summoner: NewSummoner, conn: &PgConnection) -> bool{
    if check_existing_summoner(&new_summoner.puuid, conn) {
        update_summoner(new_summoner, conn)
    } else {
        diesel::insert_into(summoners::table)
            .values(new_summoner)
            .get_result::<Summoner>(conn)
            .is_ok()
    }
}

pub fn update_summoner(new_summoner: NewSummoner, conn: &PgConnection) -> bool{
    use super::schema::summoners::dsl::{
        name as n, profile_icon_id as pi, puuid as p, revision_date as r, summoner_level as s,
    };

    diesel::update(summoners::table)
        .filter(summoners::puuid.eq(&new_summoner.puuid))
        .set((
            pi.eq(&new_summoner.profile_icon_id),
            r.eq(&new_summoner.revision_date),
            n.eq(&new_summoner.name),
            s.eq(&new_summoner.summoner_level),
        ))
        .get_result::<Summoner>(conn)
        .is_ok()
}

pub fn remove_by_id(id: i64, conn: &PgConnection) -> bool {
    diesel::delete(summoners::table)
        .filter(summoners::id.eq(id))
        .execute(conn)
        .is_ok()
}

pub fn insert_summoner_ranked(new_ranked: NewSummonerRanked, summoner_id: i64, conn: &PgConnection) -> bool{
    if check_existing_summoner_ranked(summoner_id, conn) {
        update_summoner_ranked(new_ranked, summoner_id, conn)
    } else {
        diesel::insert_into(summoner_rankeds::table)
            .values(new_ranked)
            .get_result::<SummonerRanked>(conn)
            .is_ok()
    }
    
}

pub fn update_summoner_ranked(new_ranked: NewSummonerRanked, summoner_id: i64, conn: &PgConnection) -> bool {
    diesel::update(summoner_rankeds::table)
        .filter(summoner_rankeds::summoner_id.eq(summoner_id))
        .set(&new_ranked)
        .get_result::<SummonerRanked>(conn)
        .is_ok()
}

pub fn check_existing_summoner(puuid: &str, conn: &PgConnection) -> bool {
    diesel::select(
        diesel::dsl::exists(
            summoners::table.filter(
                summoners::puuid.eq(puuid)
            )
        )
    )
    .get_result::<bool>(conn)
    .unwrap()
}

pub fn check_existing_summoner_ranked(summoner_id: i64, conn: &PgConnection) -> bool {
    diesel::select(
        diesel::dsl::exists(
            summoner_rankeds::table.filter(
                summoner_rankeds::summoner_id.eq(summoner_id))
        )
    )
    .get_result::<bool>(conn)
    .unwrap()
}