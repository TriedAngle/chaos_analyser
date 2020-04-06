use actix_web::{web, HttpRequest, HttpResponse};
use diesel::r2d2::{ConnectionManager};
use diesel::PgConnection;
use reqwest::Client;

use crate::db::{summoner as db_summoner, summoner_rankeds as db_summoner_rankeds};
use crate::models::{Summoner, NewSummoner, SummonerRanked, NewSummonerRanked};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

// ###### SUMMONER ######

/// Returns a single Summoner by its data base ID
pub async fn get_summoner_by_id(req: HttpRequest, pool: web::Data<DbPool>) -> HttpResponse {
    let conn: r2d2::PooledConnection<ConnectionManager<PgConnection>> =
        pool.get().expect("couldn't get db connection from pool");

    let id: i64 = req.match_info().get("id").unwrap_or("1").parse().unwrap();

    HttpResponse::Ok()
        .json(db_summoner::get_by_id(id, &conn).await.unwrap())
        .await
        .unwrap()
}

/// Returns a single Summoner as the puuid is unique
pub async fn get_summoner_by_puuid(req: HttpRequest, pool: web::Data<DbPool>) -> HttpResponse {
    let conn: r2d2::PooledConnection<ConnectionManager<PgConnection>> =
        pool.get().expect("couldn't get db connection from pool");

    let puuid: &str = req.match_info().get("puuid").unwrap();

    HttpResponse::Ok()
        .json(db_summoner::get_by_puuid(puuid, &conn).await.unwrap())
        .await
        .unwrap()
}

/// Returns a single Summoner as the name is unique for each region
pub async fn get_summoner_by_name_and_region(
    req: HttpRequest,
    pool: web::Data<DbPool>,
) -> HttpResponse {
    let conn: r2d2::PooledConnection<ConnectionManager<PgConnection>> =
        pool.get().expect("couldn't get db connection from pool");

    let summoner_name: &str = req
        .match_info()
        .get("summoner_name")
        .unwrap_or("µPhenolphthalein");

    let summoner_region: &str = req
        .match_info()
        .get("region")
        .unwrap_or("EUW");

    HttpResponse::Ok()
        .json(
            db_summoner::get_by_name_and_region(summoner_name, summoner_region, &conn)
                .await
                .unwrap(),
        )
        .await
        .unwrap()
}

/// Returns all Summoners
/// Do not use this function in production as the returning data could be too huge
/// TODO: use range functions instead
pub async fn get_summoner_all(req: HttpRequest, pool: web::Data<DbPool>) -> HttpResponse {
    let conn: r2d2::PooledConnection<ConnectionManager<PgConnection>> =
        pool.get().expect("couldn't get db connection from pool");

    HttpResponse::Ok().json(db_summoner::get_all(&conn).await.unwrap())
}

/// Returns a Vec<Summoner> as multiple accounts with the same name could exist
pub async fn get_summoners_by_name(req: HttpRequest, pool: web::Data<DbPool>) -> HttpResponse {
    let conn: r2d2::PooledConnection<ConnectionManager<PgConnection>> =
        pool.get().expect("couldn't get db connection from pool");

    let summoner_name: &str = req
        .match_info()
        .get("summoner_name")
        .unwrap_or("µPhenolphthalein");

    HttpResponse::Ok()
        .json(
            db_summoner::get_by_name(summoner_name, &conn)
                .await
                .unwrap(),
        )
        .await
        .unwrap()
}


pub async fn add_summoner_by_name_and_region(req: HttpRequest, pool: web::Data<DbPool>, client: web::Data<Client>) -> HttpResponse {
    let conn: r2d2::PooledConnection<ConnectionManager<PgConnection>> =
        pool.get().expect("couldn't get db connection from pool");

    let summoner_name: &str = req
        .match_info()
        .get("summoner_name")
        .unwrap_or("µPhenolphthalein");

    let summoner_region: &str = req
        .match_info()
        .get("region")
        .unwrap_or("EUW");

    let new_summoner: NewSummoner = crate::riot_api::summoner_by_name(summoner_name, summoner_region, &client).await;
    if db_summoner::insert_summoner(new_summoner, &conn).await {
        let summoner: Summoner = db_summoner::get_by_name_and_region(summoner_name, summoner_region, &conn).await.unwrap();
        HttpResponse::Ok()
        .json(summoner)
        .await
        .unwrap()
    } else {
        HttpResponse::InternalServerError().json("Error finding or inserting summoner").await.unwrap()
    }
}

// ###### SUMMONER RANKED ######

/// Returns a single SummonerRanked via the summoner id (database id from the summoner, NOT the r_summoner_id)
pub async fn get_summoner_ranked_by_summoner_id(
    req: HttpRequest,
    pool: web::Data<DbPool>,
) -> HttpResponse {
    let conn: r2d2::PooledConnection<ConnectionManager<PgConnection>> =
        pool.get().expect("couldn't get db connection from pool");

    let summoner_id: i64 = req.match_info().get("id").unwrap_or("1").parse().unwrap();

    HttpResponse::Ok().json(
        db_summoner_rankeds::get_by_summoner_id(summoner_id, &conn)
            .await
            .unwrap(),
    )
}

/// Returns all SummonerRankeds
/// Do not use this function in production as the returning data could be too huge
/// TODO: use range functions instead
pub async fn get_summoner_rankeds_all(req: HttpRequest, pool: web::Data<DbPool>) -> HttpResponse {
    let conn: r2d2::PooledConnection<ConnectionManager<PgConnection>> =
        pool.get().expect("couldn't get db connection from pool");

    HttpResponse::Ok().json(db_summoner_rankeds::get_all(&conn).await.unwrap())
}

pub async fn add_summoner_ranked_by_summoner_id(req: HttpRequest, pool: web::Data<DbPool>, client: web::Data<Client>) -> HttpResponse {
    let conn: r2d2::PooledConnection<ConnectionManager<PgConnection>> =
        pool.get().expect("couldn't get db connection from pool");

    let summoner_id: i64 = req
        .match_info()
        .get("summoner_id")
        .unwrap_or("1")
        .parse().unwrap_or(1);


    let summoner: Summoner = db_summoner::get_by_id(summoner_id, &conn).await.unwrap();
    let new_summoner_ranked: NewSummonerRanked = crate::riot_api::summoner_ranked_by_id(&summoner.r_summoner_id, summoner.id, &summoner.region, &client).await;

    if db_summoner_rankeds::insert_summoner_ranked(new_summoner_ranked, &conn).await {
        let summoner_ranked: SummonerRanked = db_summoner_rankeds::get_by_summoner_id(summoner.id, &conn).await.unwrap();
        HttpResponse::Ok()
        .json(summoner_ranked)
        .await
        .unwrap()
    } else {
        HttpResponse::InternalServerError().json("Error finding or inserting summoner").await.unwrap()
    }
}
