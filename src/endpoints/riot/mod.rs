use actix_web::web::ServiceConfig;
pub mod summoner;

pub fn endpoints(config: &mut ServiceConfig) {
    summoner::endpoints(config);
}


//      ---HEADER---
pub const SAVE_OR_UPDATE_HEADER: &str = "API-PATCH";