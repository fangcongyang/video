use super::data_source_con::*;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, QueryableByName, Insertable)]
#[diesel(table_name = crate::schema::star)]
pub struct Star {
    id: Option<i32>,
    star_name: String,
    ids: String,
    site_key: String,
    movie_type: String,
    year: String,
    note: String,
    douban_rate: String,
    has_update: String,
    last_update_time: String,
    position: f64,
    pic: String,
    area: String,
}

pub mod cmd {
    
    use super::*;
    use tauri::command;
    use diesel::{insert_into, delete, dsl::max};
    use crate::schema::star::dsl::*;

    #[command]
    pub fn select_star() -> Vec<Star> {
        let mut conn = get_once_db_conn();
        let results = star.select(Star::as_select()).load::<Star>(&mut conn).unwrap();
        results
    }

    #[command]
    pub fn get_star_by_uq(star_site_key: &str, star_ids: &str) -> Option<Star> {
        let mut conn = get_once_db_conn();
        let star_info = star.filter(site_key.is(star_site_key)).filter(ids.is(star_ids))
            .select(Star::as_select()).first::<Star>(&mut conn);
        match star_info {
            Ok(si) => Some(si),
            Err(_) => None,
        }
    }

    #[command]
    pub fn save_star(mut star_info: Star) {
        let mut conn = get_once_db_conn();
        match star_info.id {
            Some(star_info_id) => {
                let _result = diesel::update(star)
                .set((douban_rate.eq(star_info.douban_rate), has_update.eq(star_info.has_update), 
                    last_update_time.eq(star_info.last_update_time), position.eq(star_info.position)))
                .filter(id.eq(star_info_id)).execute(&mut conn);
            }
            _ => {
                let position_max = star.select(max(position)).first::<Option<f64>>(&mut conn).unwrap().unwrap_or(0.00);
                star_info.position = position_max + 10.0;
                insert_into(star).values(&star_info).execute(&mut conn).unwrap();
            }
        }
    }
    
    #[command]
    pub fn insert_stars(stars: Vec<Star>) {
        let mut conn = get_once_db_conn();
        let mut position_max = star.select(max(position)).first::<Option<f64>>(&mut conn).unwrap().unwrap_or(0.00);
        for mut star_info in stars {
            position_max = position_max + 10.0;
            star_info.position = position_max;
            insert_into(star).values(&star_info).execute(&mut conn).unwrap();
        }
    }

    #[command]
    pub fn del_star(star_id: i32) {
        let mut conn = get_once_db_conn();
        delete(star.filter(id.eq(&star_id))).execute(&mut conn).unwrap();
    }

    #[command]
    pub fn del_stars(star_ids: Vec<i32>) {
        let mut conn = get_once_db_conn();
        if star_ids.len() == 0 {
            return;
        }
        delete(star.filter(id.eq_any(&star_ids))).execute(&mut conn).unwrap();
    }

}
