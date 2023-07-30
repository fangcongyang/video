use super::data_source_con::*;
use serde::{Serialize, Deserialize};
use crate::utils;
use tokio::task; 

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Site {
    id: i32,
    key: String,
    name: String,
    api: String,
    group: String,
    isActive: String,
    status: String,
    reverseOrder: String,
}


#[tokio::main]
pub async fn check_init_site () {
    let task = task::spawn(async_check_init_site());
    task.await.unwrap();
}

async fn async_check_init_site () {
    let mut binding = CACHE.lock().unwrap();
    let conn = binding.get(DBNAME.into()).unwrap();
    let count: i32 = conn.query_row("SELECT COUNT(1) FROM site", (), |row| row.get(0)).unwrap_or(0);
    
    if count == 0 {
        let sites_str = utils::read_init_data_file("sites.json");
        let sites: Vec<Site> = serde_json::from_str(&sites_str).unwrap();
        for site in sites {
            conn.execute(
            "INSERT INTO site (key, name, api, `group`, is_active, status, reverse_order) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                (&site.key, &site.name, &site.api, &site.group, &site.isActive, &site.status, &site.reverseOrder),
            ).unwrap();
        }
    }
}

pub mod cmd {
    use super::*;
    use rusqlite::{params, named_params};
    use tauri::command;

    #[command]
    pub fn select_site() -> Vec<Site> {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        
        let mut stmt = conn.prepare("SELECT * FROM site").unwrap();
        let sites = stmt.query_map([], |row| {
            Ok(Site {
                id: row.get(0)?,
                key: row.get(1)?,
                name: row.get(2)?,
                api: row.get(3)?,
                group: row.get(4)?,
                isActive: row.get(5)?,
                status: row.get(6)?,
                reverseOrder: row.get(7)?
            })
        }).unwrap();
        let sites: Vec<Site> = sites.map(|site| site.unwrap()).collect();
        sites
    }
    
    #[command]
    pub fn get_site_by_key(key: &str) -> String {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        
        let site = conn.query_row("SELECT * FROM site where key = :key", 
          named_params! { ":key": key, },
            |row| {
                Ok(Site {
                    id: row.get(0)?,
                    key: row.get(1)?,
                    name: row.get(2)?,
                    api: row.get(3)?,
                    group: row.get(4)?,
                    isActive: row.get(5)?,
                    status: row.get(6)?,
                    reverseOrder: row.get(7)?
                })
            });
        match site { 
            Ok(s) => { serde_json::to_string(&s).unwrap() } 
            Err(_e) => { "".to_string() } 
        }
    }

    #[command]
    pub fn save_site(site: Site) {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        match Some(site.id) {
            Some(_i) => conn.execute(
                "INSERT INTO site (id, key, name, api, group, is_active, status, reverse_order) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                (&site.id, &site.key, &site.name, &site.api, &site.group, &site.isActive, &site.status, &site.reverseOrder),
            ).unwrap(),
            None => conn.execute(
                "UPDATE site SET key = ?1, name = ?2, api = ?3, group = ?4, is_active = ?5, status = ?6, reverse_order = ?7  WHERE id = ?8",
                (&site.key, &site.name, &site.api, &site.group, &site.isActive, &site.status, &site.reverseOrder, &site.id, ),
            ).unwrap(),
        };
    }

    #[command]
    pub fn del_site(id: i32) {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        conn.execute("DELETE FROM site WHERE id = ?1", params![&id]).unwrap();
    }

}
