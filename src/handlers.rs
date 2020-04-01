use actix_web::{web, HttpRequest, HttpResponse};
use diesel::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use super::riot_api;
use super::db;
use super::models::Summoner;


type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn get_summoner_by_name(pool: web::Data<DbPool>, req: HttpRequest, tmpl: web::Data<tera::Tera>) -> HttpResponse {
    let mut ctx = tera::Context::new();
    let name = req.match_info().get("name").unwrap_or("yasuo xd");
    let conn = pool.get().expect("couldn't get db connection from pool");
    let summoner: Summoner = riot_api::summoner_by_name(&name).await;
    db::add_summoner(&conn, summoner.clone()).unwrap();
    ctx.insert("summoner_name", &summoner.name);
    ctx.insert("summoner_level", &summoner.summoner_level);
    ctx.insert("summoner_icon_id", &summoner.profile_icon_id);
    let s = tmpl.render("summoner.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
}
