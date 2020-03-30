table! {
    summoners (account_id) {
        account_id -> Text,
        profile_icon_id -> Nullable<Int4>,
        revision_date -> Nullable<Int8>,
        name -> Nullable<Text>,
        id -> Nullable<Text>,
        puuid -> Nullable<Text>,
        summoner_level -> Nullable<Text>,
    }
}
