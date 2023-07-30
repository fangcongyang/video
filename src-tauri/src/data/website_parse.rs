use super::data_source_con::*;
use serde::{Serialize, Deserialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct WebsiteParse {
    id: i32,
    websiteKey: String,
    websiteParseUrl: String,
}

pub mod cmd {
    use super::*;
    use rusqlite::{params, named_params};
    use tauri::command;

    #[command]
    pub fn select_website_parse() -> Vec<WebsiteParse> {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        
        let mut stmt = conn.prepare("SELECT * FROM website_parse").unwrap();
        let website_parses = stmt.query_map([], |row| {
            Ok(WebsiteParse {
                id: row.get(0)?,
                websiteKey: row.get(1)?,
                websiteParseUrl: row.get(2)?,
            })
        }).unwrap();
        let website_parses: Vec<WebsiteParse> = website_parses.map(|website_parse| website_parse.unwrap()).collect();
        website_parses
    }
    
    #[command]
    pub fn get_website_parse_by_key(website_key: &str) -> String {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        
        let website_parse = conn.query_row("SELECT * FROM website_parse where website_key = :websiteKey", 
          named_params! { ":websiteKey": website_key, },
            |row| {
                Ok(WebsiteParse {
                    id: row.get(0)?,
                    websiteKey: row.get(1)?,
                    websiteParseUrl: row.get(2)?,
                })
            });
        match website_parse { 
            Ok(s) => { serde_json::to_string(&s).unwrap() } 
            Err(_e) => { "".to_string() } 
        }
    }

    #[command]
    pub fn save_website_parse(website_parse: WebsiteParse) {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        match Some(website_parse.id) {
            Some(_i) => conn.execute(
                "INSERT INTO website_parse (id, website_key, website_parse_url) VALUES (?1, ?2, ?3)",
                (&website_parse.id, &website_parse.websiteKey, &website_parse.websiteParseUrl, ),
            ).unwrap(),
            None => conn.execute(
                "UPDATE website_parse SET website_key = ?1, website_parse_url = ?2 WHERE id = ?3",
                (&website_parse.websiteKey, &website_parse.websiteParseUrl, &website_parse.id, ),
            ).unwrap(),
        };
    }

    #[command]
    pub fn del_website_parse(id: i32) {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        conn.execute("DELETE FROM website_parse WHERE id = ?1", params![&id]).unwrap();
    }

}
