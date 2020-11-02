pub mod summoner;

//      ---HEADER---
pub const KEY: &str = "X-Riot-Token";
//      ---SERVER---

pub const BASE_BR: &str = "https://br1.api.riotgames.com/";
pub const BASE_EUNE: &str = "https://eun1.api.riotgames.com/";
pub const BASE_EUW: &str = "https://euw1.api.riotgames.com/";
pub const BASE_JP: &str = "https://jp1.api.riotgames.com/";
pub const BASE_KR: &str = "https://kr.api.riotgames.com/";
pub const BASE_LAN: &str = "https://la1.api.riotgames.com/";
pub const BASE_LAS: &str = "https://la2.api.riotgames.com/";
pub const BASE_NA: &str = "https://na1.api.riotgames.com/";
pub const BASE_OCE: &str = "https://oc1.api.riotgames.com/";
pub const BASE_TR: &str = "https://tr1.api.riotgames.com/";
pub const BASE_RU: &str = "https://ru.api.riotgames.com/";

//      ---SUMMONER---
// summoner account id
pub const SUMMONER_BY_ACCOUNT: &str = "lol/summoner/v4/summoners/by-account/";
// summoner search
pub const SUMMONER_BY_NAME: &str = "lol/summoner/v4/summoners/by-name/";
// unique identifier
pub const SUMMONER_BY_PUUID: &str = "lol/summoner/v4/summoners/by-puuid/";
// useless if not used for leagues :/
pub const SUMMONER_BY_ID: &str = "lol/summoner/v4/summoners/";

//      ---MATCHES---
// match by match id
pub const MATCH_BY_ID: &str = "lol/match/v4/matches/";
// matches by summoner account id
pub const MATCH_BY_ACCOUNT: &str = "lol/match/v4/by-account/";
// match ids from tournament code
pub const MATCH_IDS_BY_TOURNAMENT: &str = "lol/match/v4/matches/by-tournament-code/<t_c>/ids";
// match by match id and tournament code
pub const MATCH_BY_ID_AND_TOURNAMENT: &str = "lol/match/v4/matches/<m_id>/by-tournament-code/";
// match timeline by match id
pub const MATCH_TIMELINE: &str = "lol/match/v4/timelines/by-match/";

//      ---LEAGUE---
// summoner ranked data
pub const SUMMONER_RANK_BY_SUMMONER_ID: &str = "lol/league/v4/entries/by-summoner/";
// challenger by ranked queue:
// RANKED_SOLO_5x5, RANKED_FLEX_SR, RANKED_FLEX_TT
pub const CHALLENGER_LEAGUE_BY_QUEUE: &str = "lol/league/v4/challengerleagues/by-queue/";
pub const GRANDMASTER_LEAGUE_BY_QUEUE: &str = "lol/league/v4/grandmasterleagues/by-queue/";
pub const MATER_LEAGUE_BY_QUEUE: &str = "lol/league/v4/masterleagues/by-queue/";
// summoners in 'ranked list'
pub const RANKED_SUMMONERS_BY_QUEUE_TIER_DIVISION: &str =
    "lol/league/v4/entries/<queue>/<tier>/<division>/";

//      ---OTHER---
pub const CHAMP_ROTATION: &str = "/lol/platform/v3/champion-rotationsReturns";
