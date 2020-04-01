use diesel::prelude::*;
use diesel::PgConnection;
use super::models::Summoner;

pub fn add_summoner(conn: &PgConnection, summoner: Summoner) -> Result<Summoner, diesel::result::Error> {
    use super::schema::summoners;

    if diesel::insert_into(summoners::table) 
        .values(&summoner)
        .execute(conn)
        .is_ok() {
            println!("Creating new summoner");
            Ok(summoner)
        } else {
            println!("summoner already exists -> updating summoner");
            update_summoner(conn, summoner)
        }
}

pub fn update_summoner(conn: &PgConnection, summoner: Summoner) -> Result<Summoner, diesel::result::Error> {
    use super::schema::summoners;
    use super::schema::summoners::dsl::summoners as all_summoners;
    use super::schema::summoners::dsl::{profile_icon_id as p, revision_date as r, name as n, summoner_level as s};

    diesel::update(all_summoners.find(&summoner.puuid))
        .set((p.eq(summoners::profile_icon_id), r.eq(summoners::revision_date),
            n.eq(summoners::name), s.eq(summoners::summoner_level)))
        .execute(conn)?;
    println!("summoner updated");
    Ok(summoner)

}