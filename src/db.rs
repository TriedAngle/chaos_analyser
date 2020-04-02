use super::models::Summoner;
use diesel::prelude::*;
use diesel::PgConnection;

pub fn insert_or_update_summoner(
    conn: &PgConnection,
    summoner: Summoner,
) -> Result<Summoner, diesel::result::Error> {
    use super::schema::summoners;
    use super::schema::summoners::dsl::{
        name as n, profile_icon_id as pi, puuid as p, revision_date as r, summoner_level as s,
    };

    diesel::insert_into(summoners::table)
        .values(&summoner)
        .on_conflict(p)
        .do_update()
        .set((
            pi.eq(summoners::profile_icon_id),
            r.eq(summoners::revision_date),
            n.eq(summoners::name),
            s.eq(summoners::summoner_level),
        ))
        .execute(conn)?;

    Ok(summoner)
}
