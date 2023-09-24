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