diesel::table! {
    download_info (id) {
        id -> Nullable<Integer>,
        movie_name -> Text,
        url -> Text,
        sub_title_name -> Text,
        status -> Text,
        download_count -> Integer,
        count -> Integer,
        download_status -> Text
    }
}

diesel::table! {
    star (id) {
        id -> Nullable<Integer>,
        star_name -> Text,
        ids -> Text,
        site_key -> Text,
        movie_type -> Text,
        year -> Text,
        note -> Text,
        douban_rate -> Text,
        has_update -> Text,
        last_update_time -> Text,
        position -> Double,
        pic -> Text,
        area -> Text,
    }
}

diesel::table! {
    website_parse (id) {
        id -> Nullable<Integer>,
        website_key -> Text,
        website_parse_url -> Text,
        position -> Double,
    }
}