use super::data_source_con::*;
use serde::{Serialize, Deserialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct History {
    id: i32,
    name: String,
    ids: String,
    index: i32,
    startPosition: i32,
    endPosition: i32,
    playTime: f64,
    siteKey: String,
    onlinePlay: String,
    detail: String,
    videoFlag: String,
    duration: f64,
    hasUpdate: String,
    updateTime: String,
}

pub mod cmd {
    use crate::utils;

    use super::*;
    use rusqlite::{params, named_params, params_from_iter};
    use tauri::command;

    #[command]
    pub fn select_history() -> Vec<History> {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        
        let mut stmt = conn.prepare("SELECT * FROM history ORDER BY update_time DESC").unwrap();
        let historys = stmt.query_map([], |row| {
            Ok(History {
                id: row.get(0)?,
                name: row.get(1)?,
                ids: row.get(2)?,
                index: row.get(3)?,
                startPosition: row.get(4)?,
                endPosition: row.get(5)?,
                playTime: row.get(6)?,
                siteKey: row.get(7)?,
                onlinePlay: row.get(8)?,
                detail: row.get(9)?,
                videoFlag: row.get(10)?,
                duration: row.get(11)?,
                hasUpdate: row.get(12)?,
                updateTime: row.get(13)?,
            })
        }).unwrap();
        
        let history_list: Vec<History> = historys.map(|history| history.unwrap()).collect();
        history_list
    }

    #[command]
    pub fn get_history_by_uq(site_key: &str, ids: &str) -> String {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        let history = conn.query_row("SELECT * FROM history where site_key = :siteKey and ids = :ids", 
          named_params! { ":siteKey": site_key, ":ids": ids },
            |row| {
                Ok(History {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    ids: row.get(2)?,
                    index: row.get(3)?,
                    startPosition: row.get(4)?,
                    endPosition: row.get(5)?,
                    playTime: row.get(6)?,
                    siteKey: row.get(7)?,
                    onlinePlay: row.get(8)?,
                    detail: row.get(9)?,
                    videoFlag: row.get(10)?,
                    duration: row.get(11)?,
                    hasUpdate: row.get(12)?,
                    updateTime: row.get(13)?,
                })
            });
        
        match history { 
            Ok(his) => { serde_json::to_string(&his).unwrap() } 
            Err(_e) => { "".to_string() } 
        }
    }

    #[command]
    pub fn save_history(history: History) {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        if history.id == 0 {
            conn.execute(
                "INSERT INTO history (name, ids, `index`, start_position, end_position, play_time, site_key, online_play, detail, video_flag, duration, 
                    has_update, update_time) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
                (&history.name, &history.ids, &history.index, &history.startPosition, &history.endPosition, &history.playTime, &history.siteKey, 
                    &history.onlinePlay, &history.detail, &history.videoFlag, &history.duration, &history.hasUpdate, &history.updateTime),
            ).unwrap();
        } else {
            conn.execute(
                "UPDATE history SET `index` = ?1, start_position = ?2, end_position = ?3, play_time = ?4, detail = ?5, video_flag = ?6, duration = ?7,
                 has_update = ?8, update_time = ?9 WHERE id = ?10",
                (&history.index, &history.startPosition, &history.endPosition, &history.playTime, &history.detail, &history.videoFlag, &history.duration, 
                    &history.hasUpdate, &history.updateTime, &history.id),
            ).unwrap();
        }
    }

    #[command]
    pub fn insert_historys(historys: Vec<History>) {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        for history in historys {
            conn.execute(
                "INSERT INTO history (name, ids, `index`, start_position, end_position, play_time, site_key, online_play, detail, video_flag, duration,
                    has_update, update_time)
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
                (&history.name, &history.ids, &history.index, &history.startPosition, &history.endPosition, &history.playTime, &history.siteKey, 
                    &history.onlinePlay, &history.detail, &history.videoFlag, &history.duration, &history.hasUpdate, &history.updateTime),
            ).unwrap();
        }
    }

    #[command]
    pub fn del_history(id: i32) {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        conn.execute("DELETE FROM history WHERE id = ?1", params![&id]).unwrap();
    }

    #[command]
    pub fn del_all_history() {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        conn.execute("DELETE FROM history WHERE 1 = 1", []).unwrap();
    }

    #[command]
    pub fn del_historys(ids: Vec<i32>) {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        if ids.len() == 0 {
            return;
        }
        let vars = utils::repeat_vars(ids.len());
        let sql = format!(
            "DELETE FROM history WHERE id IN ({})",
            vars,);
        let mut stmt = conn.prepare(&sql).unwrap();
        stmt.execute(params_from_iter(&ids)).unwrap();
    }
}
