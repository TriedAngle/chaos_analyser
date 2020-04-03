use super::db;
use super::models::{Summoner, SummonerRanked};
use super::riot_api;
use actix_web::{web, HttpRequest, HttpResponse};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use reqwest::Client;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn summoner_page(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    tmpl: web::Data<tera::Tera>,
    client: web::Data<Client>,
) -> HttpResponse {
    let mut ctx: tera::Context = tera::Context::new();
    let region: &str = req.match_info().get("region").unwrap_or("EUW");
    let name: &str = req
        .match_info()
        .get("summoner_name")
        .unwrap_or("µPhenolphthalein");
    let conn: r2d2::PooledConnection<ConnectionManager<PgConnection>> =
        pool.get().expect("couldn't get db connection from pool");

    let summoner: Summoner = riot_api::summoner_by_name(&name, &region, &client).await;
    let summoner_ranked: SummonerRanked = riot_api::summoner_ranked_by_id(
        riot_api::get_summoner_id_from_summoner(&summoner).await,
        &region,
        &client,
    )
    .await;

    db::insert_or_update_summoner(&conn, summoner.clone()).unwrap();

    ctx.insert("summoner_name", &summoner.name);
    ctx.insert("summoner_level", &summoner.summoner_level);
    ctx.insert("summoner_icon_id", &summoner.profile_icon_id);
    ctx.insert("summoner_ranked_rank", &summoner_ranked.rank);
    ctx.insert("summoner_ranked_wins", &summoner_ranked.wins);

    let s = tmpl.render("summoner.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
}
