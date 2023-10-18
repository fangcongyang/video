use super::data_source_con::*;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, QueryableByName, Insertable)]
#[diesel(table_name = crate::schema::search_record)]
pub struct SearchRecord {
    id: Option<i32>,
    keywords: String
}

pub mod cmd {
    use super::*;
    use tauri::command;
    use diesel::{insert_into, delete};
    use crate::schema::search_record::dsl::*;

    #[command]
    pub fn select_search_record() -> Vec<SearchRecord> {
        let mut conn = get_once_db_conn();
        let search_records = search_record.select(SearchRecord::as_select()).order(id.desc()).load::<SearchRecord>(&mut conn).unwrap();
        search_records
    }

    #[command]
    pub fn save_search_record(search_record_info: SearchRecord) {
        let mut conn = get_once_db_conn();
        let count = search_record.filter(keywords.is(&search_record_info.keywords)).count().get_result::<i64>(&mut conn).unwrap();
        if count == 0 {
            insert_into(search_record).values(&search_record_info).execute(&mut conn).unwrap();
        }
    }

    #[command]
    pub fn del_all_search_record() {
        let mut conn = get_once_db_conn();
        delete(search_record).execute(&mut conn).unwrap();
    }

}
