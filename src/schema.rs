table! {
    summoners (puuid) {
        puuid -> Text,
        account_id -> Nullable<Text>,
        profile_icon_id -> Nullable<Int4>,
        revision_date -> Nullable<Int8>,
        name -> Nullable<Text>,
        id -> Nullable<Text>,
        summoner_level -> Nullable<Int8>,
    }
}
