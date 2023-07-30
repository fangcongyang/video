use super::data_source_con::*;
use serde::{Serialize, Deserialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Star {
    id: i32,
    name: String,
    ids: String,
    siteKey: String,
    movieType: String,
    year: String,
    note: String,
    doubanRate: String,
    hasUpdate: String,
    lastUpdateTime: String,
    position: f64,
    pic: String,
    area: String,
}

pub mod cmd {
    use crate::utils;

    use super::*;
    use rusqlite::{params, named_params, params_from_iter};
    use tauri::command;

    #[command]
    pub fn select_star() -> Vec<Star> {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        
        let mut stmt = conn.prepare("SELECT * FROM star").unwrap();
        let stars = stmt.query_map([], |row| {
            Ok(Star {
                id: row.get(0)?,
                name: row.get(1)?,
                ids: row.get(2)?,
                siteKey: row.get(3)?,
                movieType: row.get(4)?,
                year: row.get(5)?,
                note: row.get(6)?,
                doubanRate: row.get(7)?,
                hasUpdate: row.get(8)?,
                lastUpdateTime: row.get(9)?,
                position: row.get(10)?,
                pic: row.get(11)?,
                area: row.get(12)?,
            })
        }).unwrap();
        
        let star_list: Vec<Star> = stars.map(|star| star.unwrap()).collect();
        star_list
    }

    #[command]
    pub fn get_star_by_uq(site_key: &str, ids: &str) -> String {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        let star = conn.query_row("SELECT * FROM star where site_key = :siteKey and ids = :ids", 
          named_params! { ":siteKey": site_key, ":ids": ids },
            |row| {
                Ok(Star {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    ids: row.get(2)?,
                    siteKey: row.get(3)?,
                    movieType: row.get(4)?,
                    year: row.get(5)?,
                    note: row.get(6)?,
                    doubanRate: row.get(7)?,
                    hasUpdate: row.get(8)?,
                    lastUpdateTime: row.get(9)?,
                    position: row.get(10)?,
                    pic: row.get(11)?,
                    area: row.get(12)?,
                })
            });
        
        match star { 
            Ok(s) => { serde_json::to_string(&s).unwrap() } 
            Err(_e) => { "".to_string() } 
        }
    }

    #[command]
    pub fn save_star(mut star: Star) {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        if star.id == 0 {
            let position: f64 = conn.query_row("SELECT MAX(position) FROM star", (), |row| row.get(0)).unwrap_or(0.0);
            star.position = position + 10.0;
            conn.execute(
                "INSERT INTO star (name, ids, site_key, movie_type, year, note, douban_rate, has_update, last_update_time, position, pic, area)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                (&star.name, &star.ids, &star.siteKey, &star.movieType, &star.year, &star.note, &star.doubanRate, &star.hasUpdate, &star.lastUpdateTime, 
                &star.position, &star.pic, &star.area),
            ).unwrap();
        } else {
            conn.execute(
                "UPDATE star SET douban_rate = ?1, has_update = ?2, last_update_time = ?3, position = ?4 WHERE id = ?5",
                (&star.doubanRate, &star.hasUpdate, &star.lastUpdateTime, &star.position, &star.id),
            ).unwrap();
        }
    }
    
    #[command]
    pub fn insert_stars(stars: Vec<Star>) {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        for mut star in stars {
            let position: f64 = conn.query_row("SELECT MAX(position) FROM star", (), |row| row.get(0)).unwrap_or(0.0);
            star.position = position + 10.0;
            conn.execute(
                "INSERT INTO star (name, ids, site_key, movie_type, year, note, douban_rate, has_update, last_update_time, position, pic, area)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                (&star.name, &star.ids, &star.siteKey, &star.movieType, &star.year, &star.note, &star.doubanRate, &star.hasUpdate, &star.lastUpdateTime, 
                &star.position, &star.pic, &star.area),
            ).unwrap();
        }
    }

    #[command]
    pub fn del_star(id: i32) {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        conn.execute("DELETE FROM star WHERE id = ?1", params![&id]).unwrap();
    }

    #[command]
    pub fn del_stars(ids: Vec<i32>) {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        if ids.len() == 0 {
            return;
        }
        let vars = utils::repeat_vars(ids.len());
        let sql = format!(
            "DELETE FROM star WHERE id IN ({})",
            vars,);
        let mut stmt = conn.prepare(&sql).unwrap();
        stmt.execute(params_from_iter(&ids)).unwrap();
    }

}
