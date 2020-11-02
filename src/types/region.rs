use crate::riot::api::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum Region {
    BR,
    EUNE,
    EUW,
    JP,
    KR,
    LAN,
    LAS,
    NA,
    OCE,
    TR,
    RU,
}

impl Region {
    pub fn as_str(&self) -> &str {
        match *self {
            Region::BR => BASE_BR,
            Region::EUNE => BASE_EUNE,
            Region::EUW => BASE_EUW,
            Region::JP => BASE_JP,
            Region::KR => BASE_KR,
            Region::LAN => BASE_LAN,
            Region::LAS => BASE_LAS,
            Region::NA => BASE_NA,
            Region::OCE => BASE_OCE,
            Region::TR => BASE_TR,
            Region::RU => BASE_RU,
        }
    }
}
