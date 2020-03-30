-- Your SQL goes here
CREATE TABLE summoners (
    account_id TEXT PRIMARY KEY,
    profile_icon_id INTEGER,
    revision_date BIGINT,
    name TEXT,
    id TEXT,
    puuid TEXT,
    summoner_level TEXT
)