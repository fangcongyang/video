use super::data_source_con::*;
use serde::{Serialize, Deserialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRecord {
    id: i32,
    keywords: String
}

pub mod cmd {
    use super::*;
    use rusqlite::named_params;
    use tauri::command;

    #[command]
    pub fn select_search_record() -> Vec<SearchRecord> {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        
        let mut stmt = conn.prepare("SELECT * FROM search_record").unwrap();
        let search_records = stmt.query_map([], |row| {
            Ok(SearchRecord {
                id: row.get(0)?,
                keywords: row.get(1)?,
            })
        }).unwrap();
        
        let search_record_list: Vec<SearchRecord> = search_records.map(|search_record| search_record.unwrap()).collect();
        search_record_list
    }

    #[command]
    pub fn save_search_record(search_record: SearchRecord) {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        let mut stmt = conn.prepare("SELECT COUNT(1) FROM search_record WHERE keywords = :keywords").unwrap();
        let count: i32 = stmt.query_row(named_params! { ":keywords": &search_record.keywords, }, |row| row.get(0)).unwrap();
        if count == 0 {
            conn.execute(
                "INSERT INTO search_record (keywords) VALUES (?1)",
                (&search_record.keywords, ),
            ).unwrap();
        }
    }

    #[command]
    pub fn del_all_search_record() {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        conn.execute("DELETE FROM search_record WHERE 1 = 1", {}).unwrap();
    }

}
