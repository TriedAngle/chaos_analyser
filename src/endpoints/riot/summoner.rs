use actix_web::{get, web, Error, HttpResponse, HttpRequest};
use actix_web::web::ServiceConfig;
use crate::riot::api;
use crate::db::PgPool;
use reqwest::Client;
use crate::types::region::Region;
use crate::config::Config;
use crate::endpoints::riot::SAVE_OR_UPDATE_HEADER;

pub fn endpoints(config: &mut ServiceConfig) {
    config
        .service(summoner_by_id);
}

#[get("/api/riot/summoners/{region}/{id}")]
pub async fn summoner_by_id(
    pool: web::Data<PgPool>,
    client: web::Data<Client>,
    config: web::Data<Config>,
    web::Path((region, id)): web::Path<(Region, String)>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    println!("id: {:?}", id);
    println!("region: {:?}", region);
    let client: &Client = client.get_ref();
    println!("key: {:?}", config.api_key);
    let item = api::summoner::summoner_by_id(id, region, client, &config.api_key).await.unwrap();
    match req.headers().get(SAVE_OR_UPDATE_HEADER) {
        Some(value) => {
            let value = value.to_str().unwrap();
            if value == "true" {
                // save to database
            }
        }
        None => {}
    }
    println!("item: {:?}", item);
    Ok(HttpResponse::Ok().json(item))
}