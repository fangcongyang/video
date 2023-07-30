use super::data_source_con::*;
use serde::{Serialize, Deserialize};
use crate::utils;
use tokio::task; 

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Shortcut {
    id: i32,
    key: String,
    name: String,
    desc: String,
}


#[tokio::main]
pub async fn check_init_shortcut () {
    let task = task::spawn(async_check_init_shortcut());
    task.await.unwrap();
}

async fn async_check_init_shortcut () {
    let mut binding = CACHE.lock().unwrap();
    let conn = binding.get(DBNAME.into()).unwrap();
    let count: i32 = conn.query_row("SELECT COUNT(1) FROM shortcut", (), |row| row.get(0)).unwrap_or(0);
    
    if count == 0 {
        let shortcut_strs = utils::read_init_data_file("shortcut.json");
        let shortcuts: Vec<Shortcut> = serde_json::from_str(&shortcut_strs).unwrap();
        for shortcut in shortcuts {
            conn.execute(
            "INSERT INTO shortcut (key, name, desc) VALUES (?1, ?2, ?3)",
                (&shortcut.key, &shortcut.name, &shortcut.desc),
            ).unwrap();
        }
    }
}

pub mod cmd {
    use super::*;
    use tauri::command;

    #[command]
    pub fn select_shortcut() -> Vec<Shortcut> {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        
        let mut stmt = conn.prepare("SELECT * FROM shortcut").unwrap();
        let shortcuts = stmt.query_map([], |row| {
            Ok(Shortcut {
                id: row.get(0)?,
                key: row.get(1)?,
                name: row.get(2)?,
                desc: row.get(3)?
            })
        }).unwrap();
        let shortcuts: Vec<Shortcut> = shortcuts.map(|shortcut| shortcut.unwrap()).collect();
        shortcuts
    }

    #[command]
    pub fn save_shortcut(shortcut: Shortcut) {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        if shortcut.id > 0 {
            conn.execute(
                "UPDATE shortcut SET key = ?1 WHERE id = ?2",
                (&shortcut.key, &shortcut.id, ),
            ).unwrap();
        }
    }

}
