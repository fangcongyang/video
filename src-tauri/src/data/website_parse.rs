use super::data_source_con::*;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, QueryableByName, Insertable)]
#[diesel(table_name = crate::schema::website_parse)]
pub struct WebsiteParse {
    id: Option<i32>,
    website_key: String,
    website_parse_url: String,
    position: f64,
}

pub mod cmd {
    use super::*;
    use diesel::{dsl::max, update, insert_into, delete};
    use tauri::command;
    use crate::schema::website_parse::dsl::*;

    #[command]
    pub fn select_website_parse() -> Vec<WebsiteParse> {
        let mut conn = get_once_db_conn();
        let results = website_parse.select(WebsiteParse::as_select()).load::<WebsiteParse>(&mut conn).unwrap();
        results
    }

    #[command]
    pub fn save_website_parse(mut website_parse_info: WebsiteParse) {
        let mut conn = get_once_db_conn();
        match website_parse_info.id {
            Some(website_parse_info_id) => {
                let _result = update(website_parse)
                .set((website_key.eq(website_parse_info.website_key), website_parse_url.eq(website_parse_info.website_parse_url)))
                .filter(id.eq(website_parse_info_id)).execute(&mut conn);
            }
            _ => {
                let position_max = website_parse.select(max(position)).first::<Option<f64>>(&mut conn).unwrap().unwrap_or(0.00);
                website_parse_info.position = position_max + 10.0;
                insert_into(website_parse).values(&website_parse_info).execute(&mut conn).unwrap();
            }
        }
    }

    #[command]
    pub fn del_website_parse(website_parse_info_id: i32) {
        let mut conn = get_once_db_conn();
        delete(website_parse.filter(id.eq(&website_parse_info_id))).execute(&mut conn).unwrap();
    }

}
