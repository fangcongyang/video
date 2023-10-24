use super::data_source_con::*;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, QueryableByName, Insertable)]
#[diesel(table_name = crate::schema::history)]
pub struct History {
    id: Option<i32>,
    history_name: String,
    ids: String,
    index: i32,
    start_position: i32,
    end_position: i32,
    play_time: f64,
    site_key: String,
    online_play: String,
    detail: String,
    video_flag: String,
    duration: f64,
    has_update: String,
    update_time: String,
}

pub mod cmd {
    use super::*;
    use tauri::command;
    use diesel::{insert_into, delete, update};
    use crate::schema::history::dsl::*;

    #[command]
    pub fn select_history() -> Vec<History> {
        let mut conn = get_once_db_conn();
        let historys = history.select(History::as_select()).order(update_time.desc()).load::<History>(&mut conn).unwrap();
        historys
    }

    #[command]
    pub fn get_history_by_uq(site_key_str: &str, ids_str: &str) -> Option<History> {
        let mut conn = get_once_db_conn();
        let history_info = history.filter(site_key.is(site_key_str)).filter(ids.is(ids_str))
            .select(History::as_select()).first::<History>(&mut conn);
        match history_info {
            Ok(hi) => Some(hi),
            Err(_) => None,
        }
    }

    #[command]
    pub fn save_history(history_info: History) {
        let mut conn = get_once_db_conn();
        match history_info.id {
            Some(history_info_id) => {
                let _result = update(history)
                .set((index.eq(history_info.index), start_position.eq(history_info.start_position), end_position.eq(history_info.end_position),
                    play_time.eq(history_info.play_time), detail.eq(history_info.detail), video_flag.eq(history_info.video_flag), 
                    duration.eq(history_info.duration), has_update.eq(history_info.has_update), update_time.eq(history_info.update_time)))
                .filter(id.eq(history_info_id)).execute(&mut conn);
            }
            _ => {
                insert_into(history).values(&history_info).execute(&mut conn).unwrap();
            }
        }
    }

    #[command]
    pub fn insert_historys(historys: Vec<History>) {
        for mut history_info in historys {
            history_info.id = None;
            save_history(history_info);
        }
    }

    #[command]
    pub fn del_history(history_id: i32) {
        let mut conn = get_once_db_conn();
        delete(history.filter(id.eq(&history_id))).execute(&mut conn).unwrap();
    }

    #[command]
    pub fn del_all_history() {
        let mut conn = get_once_db_conn();
        delete(history).execute(&mut conn).unwrap();
    }

    #[command]
    pub fn del_historys(history_ids: Vec<i32>) {
        let mut conn = get_once_db_conn();
        if history_ids.len() == 0 {
            return;
        }
        delete(history.filter(id.eq_any(&history_ids))).execute(&mut conn).unwrap();
    }
}
