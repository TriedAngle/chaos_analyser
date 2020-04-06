use crate::models::{NewSummoner, RiotSummonerRanked};
use actix_web::{web, HttpRequest, HttpResponse};
use reqwest::Client;

// Displays all Summoner Information received from the Riot API
pub async fn summoner_page(
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

    let riot_summoner: NewSummoner =
        crate::riot_api::summoner_by_name(&name, &region, &client).await;
    let riot_summoner_ranked: RiotSummonerRanked =
        crate::riot_api::riot_summoner_ranked_by_r_summoner_id(
            &riot_summoner.r_summoner_id,
            &riot_summoner.region,
            &client,
        )
        .await;

    ctx.insert("error", "");
    ctx.insert(
        "s_games",
        &(riot_summoner_ranked.s_wins + riot_summoner_ranked.s_losses),
    );
    ctx.insert(
        "f_games",
        &(riot_summoner_ranked.f_wins + riot_summoner_ranked.f_losses),
    );
    ctx.insert("summoner", &riot_summoner);
    ctx.insert("summoner_ranked", &riot_summoner_ranked);

    let s = tmpl.render("views/riot_summoner.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
}
