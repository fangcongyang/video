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

diesel::table! {
    history (id) {
        id -> Nullable<Integer>,
        history_name -> Text,
        ids -> Text,
        index -> Integer,
        start_position -> Integer,
        end_position -> Integer,
        play_time -> Double,
        site_key -> Text,
        online_play -> Text,
        detail -> Text,
        video_flag -> Text,
        duration -> Double,
        has_update -> Text,
        update_time -> Text,
    }
}

diesel::table! {
    site (id) {
        id -> Nullable<Integer>,
        site_key -> Text,
        site_name -> Text,
        api -> Text,
        site_group -> Text,
        is_active -> Text,
        status -> Text,
        position -> Nullable<Double>,
        is_reverse_order -> Text,
    }
}