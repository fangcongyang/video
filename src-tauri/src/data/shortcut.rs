use super::data_source_con::*;
use serde::{Serialize, Deserialize};
use tokio::task; 
use diesel::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, QueryableByName, Insertable)]
#[diesel(table_name = crate::schema::shortcut)]
pub struct Shortcut {
    id: Option<i32>,
    key: String,
    name: String,
    desc: String,
}

#[tokio::main]
pub async fn check_init_shortcut () {
    let task = task::spawn(cmd::async_check_init_shortcut());
    task.await.unwrap();
}

pub mod cmd {
    use super::*;
    use diesel::{insert_into, delete};
    use tauri::command;
    use crate::{schema::shortcut::dsl::*, utils};

    pub async fn async_check_init_shortcut () {
        let mut conn = get_once_db_conn();
        let count = shortcut.count().get_result::<i64>(&mut conn).unwrap();
        
        if count == 0 {
            let shortcut_strs = utils::read_init_data_file("shortcut.json");
            let shortcuts: Vec<Shortcut> = serde_json::from_str(&shortcut_strs).unwrap();
            insert_into(shortcut).values(&shortcuts).execute(&mut conn).unwrap();
        }
    }

    #[tokio::main]
    #[command]
    pub async fn reset_shortcut() {
        let mut conn = get_once_db_conn();
        delete(shortcut).execute(&mut conn).unwrap();
        let task = task::spawn(cmd::async_check_init_shortcut());
        task.await.unwrap();
    }

    #[command]
    pub fn select_shortcut() -> Vec<Shortcut> {
        let mut conn = get_once_db_conn();
        let shortcuts = shortcut.select(Shortcut::as_select()).load::<Shortcut>(&mut conn).unwrap();
        shortcuts
    }

    #[command]
    pub fn save_shortcut(shortcut_info: Shortcut) {
        let mut conn = get_once_db_conn();
        match shortcut_info.id {
            Some(shortcut_info_id) => {
                let _result = diesel::update(shortcut)
                .set(key.eq(shortcut_info.key))
                .filter(id.eq(shortcut_info_id)).execute(&mut conn);
            }
            _ => {}
        }
    }

}
