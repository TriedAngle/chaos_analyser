use super::db;
use super::models::{NewSummoner, NewSummonerRanked, Summoner, SummonerRanked};
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

    let new_summoner: NewSummoner = riot_api::summoner_by_name(&name, &region, &client).await;

    //db::insert_or_update_summoner( new_summoner.clone(), &conn);
    db::insert_summoner( new_summoner.clone(), &conn);

    let summoner: Summoner = db::get_by_puuid(&new_summoner.puuid, &conn).unwrap();

    let new_summoner_ranked: NewSummonerRanked = riot_api::summoner_ranked_by_id(
        riot_api::get_summoner_id_from_summoner(&summoner).await,
        summoner.id,
        &region,
        &client,
    )
    .await;

    db::insert_summoner_ranked(new_summoner_ranked.clone(), summoner.id, &conn);

    println!("{:?}", summoner);
    println!("{:?}", new_summoner_ranked);

    ctx.insert("summoner_name", &new_summoner.name);
    ctx.insert("summoner_level", &new_summoner.summoner_level);
    ctx.insert("summoner_icon_id", &new_summoner.profile_icon_id);
    ctx.insert("summoner_ranked_rank", &new_summoner_ranked.s_rank);
    ctx.insert("summoner_ranked_wins", &new_summoner_ranked.s_wins);

    let s = tmpl.render("summoner.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
}
