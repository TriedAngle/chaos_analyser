-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE summoners (
    puuid TEXT PRIMARY KEY,
    account_id TEXT,
    profile_icon_id INTEGER,
    revision_date BIGINT,
    name TEXT,
    summoner_id TEXT,
    summoner_level BIGINT
)