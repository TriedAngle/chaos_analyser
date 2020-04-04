table! {
    summoner_rankeds (id) {
        id -> Int8,
        summoner_id -> Int8,
        s_tier -> Text,
        f_tier -> Text,
        s_rank -> Text,
        f_rank -> Text,
        s_league_points -> Int4,
        f_league_points -> Int4,
        s_wins -> Int4,
        f_wins -> Int4,
        s_losses -> Int4,
        f_losses -> Int4,
        s_hot_streak -> Bool,
        f_hot_streak -> Bool,
        s_veteran -> Bool,
        f_veteran -> Bool,
        s_fresh_blood -> Bool,
        f_fresh_blood -> Bool,
        s_inactive -> Bool,
        f_inactive -> Bool,
        s_is_ms -> Bool,
        f_is_ms -> Bool,
        s_ms_w -> Int4,
        f_ms_w -> Int4,
        s_ms_l -> Int4,
        f_ms_l -> Int4,
        s_ms_prg -> Text,
        f_ms_prg -> Text,
        s_ms_trg -> Int4,
        f_ms_trg -> Int4,
    }
}

table! {
    summoners (id) {
        id -> Int8,
        puuid -> Text,
        account_id -> Text,
        profile_icon_id -> Int4,
        revision_date -> Int8,
        name -> Text,
        r_summoner_id -> Text,
        summoner_level -> Int8,
        region -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    summoner_rankeds,
    summoners,
);
