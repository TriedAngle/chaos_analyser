use crate::models::{Summoner, SummonerRanked};
use actix_web::{web, HttpRequest, HttpResponse};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use reqwest::Client;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn summoner_page(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    tmpl: web::Data<tera::Tera>,
    client: web::Data<Client>,
) -> HttpResponse {
    let mut ctx: tera::Context = tera::Context::new();
    let conn: r2d2::PooledConnection<ConnectionManager<PgConnection>> =
        pool.get().expect("couldn't get db connection from pool");
    let region: &str = req.match_info().get("region").unwrap_or("EUW");
    let name: &str = req
        .match_info()
        .get("summoner_name")
        .unwrap_or("µPhenolphthalein");

    let summoner: Summoner =
        match crate::db::summoner::get_by_name_and_region(&name, &region, &conn).await {
            Ok(summoner) => {
                ctx.insert("error", "");
                summoner
            },
            Err(_) => {
                ctx.insert("error", "user does not exist");
                Summoner::create_empty()
            },
        };

    let summoner_ranked: SummonerRanked =
         match crate::db::summoner_rankeds::get_by_summoner_id(summoner.id, &conn).await {
            Ok(summoner_ranked) => summoner_ranked,
            Err(_) => SummonerRanked::create_empty(),
         };

    ctx.insert("s_games", &(summoner_ranked.s_wins + summoner_ranked.s_losses));
    ctx.insert("f_games", &(summoner_ranked.f_wins + summoner_ranked.f_losses));
    ctx.insert("summoner", &summoner);
    ctx.insert("summoner_ranked", &summoner_ranked);

    let s = tmpl.render("views/db_summoner.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
}
