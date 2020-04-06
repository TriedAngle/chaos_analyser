#![allow(dead_code)]

use diesel::prelude::*;
use diesel::{PgConnection, QueryResult};

use crate::models::{NewSummoner, Summoner};

use crate::schema::summoners;
use crate::schema::summoners::dsl::summoners as all_summoners;

/// returning all summoners
/// it is strongly advised to not use this function in production
/// as huge amount of data could exist
/// alternative is get_within_range_x
/// these alternative function return a range of values
/// but: they are not added yet:
/// TODO: ADD RANGE FUNCTIONS
pub async fn get_all(conn: &PgConnection) -> QueryResult<Vec<Summoner>> {
    all_summoners
        .order(summoners::id.asc())
        .load::<Summoner>(conn)
}

/// returning a summoner by its id
pub async fn get_by_id(id: i64, conn: &PgConnection) -> QueryResult<Summoner> {
    all_summoners.find(id).get_result::<Summoner>(conn)
}

/// returning only one Summoner as no summoners have the same puuid crossregional
pub async fn get_by_puuid(puuid: &str, conn: &PgConnection) -> QueryResult<Summoner> {
    all_summoners
        .filter(summoners::puuid.eq(puuid))
        .get_result::<Summoner>(conn)
}

/// returning Vector of Summoners as the same name can exist once per region
pub async fn get_by_name(summoner_name: &str, conn: &PgConnection) -> QueryResult<Vec<Summoner>> {
    all_summoners
        .filter(summoners::name.eq(summoner_name))
        .load::<Summoner>(conn)
}

/// return only one Summoner as only one summoner per region has the same name
pub async fn get_by_name_and_region(
    summoner_name: &str,
    summoner_region: &str,
    conn: &PgConnection,
) -> QueryResult<Summoner> {
    all_summoners
        .filter(
            summoners::name
                .eq(summoner_name)
                .and(summoners::region.eq(summoner_region)),
        )
        .get_result::<Summoner>(conn)
}

/// inserts a NewSummoner or
/// updates a Summoner
///
/// avoids duplicates (uses check_existing_summoner via puuid)
pub async fn insert_summoner(new_summoner: NewSummoner, conn: &PgConnection) -> bool {
    if check_summoner_exists_by_puuid(&new_summoner.puuid, conn).await {
        update_summoner(new_summoner, conn).await
    } else {
        diesel::insert_into(summoners::table)
            .values(new_summoner)
            .get_result::<Summoner>(conn)
            .is_ok()
    }
}

/// updates summoner
/// calling this function over insert summoner is obsolete as insert executes this function automatically
/// if the summoner with the same puuid already exists
/// some imports are only imported here as they are not required for all other functions
pub async fn update_summoner(new_summoner: NewSummoner, conn: &PgConnection) -> bool {
    use crate::schema::summoners::dsl::{
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

/// a helper function to check whether to update or insert a NewSummoner / to avoid duplicates
/// using puuid instead of id as id is not known when inserting and puuid is also a unique 'identifier'
pub async fn check_summoner_exists_by_puuid(puuid: &str, conn: &PgConnection) -> bool {
    diesel::select(diesel::dsl::exists(
        summoners::table.filter(summoners::puuid.eq(puuid)),
    ))
    .get_result::<bool>(conn).unwrap()
}

pub async fn check_summoner_exists_by_name_and_region(
    summoner_name: &str,
    summoner_region: &str,
    conn: &PgConnection,
) -> bool {
    all_summoners
        .filter(
            summoners::name
                .eq(summoner_name)
                .and(summoners::region.eq(summoner_region)),
        )
        .get_result::<Summoner>(conn)
        .is_ok()
}
