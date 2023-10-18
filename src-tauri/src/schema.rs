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

diesel::table! {
    shortcut (id) {
        id -> Nullable<Integer>,
        key -> Text,
        name -> Text,
        desc -> Text,
    }
}

diesel::table! {
    channel_group (id) {
        id -> Nullable<Integer>,
        channel_name -> Text,
        channel_group_name -> Text,
        channel_active -> Text,
        channel_status -> Text,
        position -> Nullable<Double>,
        channels -> Text,
    }
}

diesel::table! {
    search_record (id) {
        id -> Nullable<Integer>,
        keywords -> Text,
    }
}