use actix_web::{web, HttpRequest, Responder};
use diesel::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use super::riot_api;
use super::db;
use super::models::Summoner;


type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn get_summoner_by_name(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("yasuo xd");
    let conn = pool.get().expect("couldn't get db connection from pool");
    let summoner: Summoner = riot_api::summoner_by_name(&name).await;
    db::add_summoner(&conn, summoner.clone()).unwrap();
    format!("summoner name: {} | {}", &summoner.name, &summoner.summoner_level)
}
