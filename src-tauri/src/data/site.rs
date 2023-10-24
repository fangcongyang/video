use super::data_source_con::*;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, QueryableByName, Insertable)]
#[diesel(table_name = crate::schema::site)]
pub struct Site {
    id: Option<i32>,
    site_key: String,
    site_name: String,
    api: String,
    site_group: String,
    is_active: String,
    status: String,
    position: Option<f64>,
    is_reverse_order: String,
}

pub fn check_init_site () {
}

pub mod cmd {
    use super::*;
    use diesel::{insert_into, update, dsl::max, delete};
    use tauri::command;
    use crate::{schema::site::dsl::*, utils};

    pub fn check_init_site() {
        let mut conn = get_once_db_conn();
        let count = site.count().get_result::<i64>(&mut conn).unwrap();
        
        if count == 0 {
            let sites_str = utils::read_init_data_file("sites.json");
            let mut sites: Vec<Site> = serde_json::from_str(&sites_str).unwrap();
            let sites_18_str = utils::read_init_data_file("18+sites.json");
            let sites_18: Vec<Site> = serde_json::from_str(&sites_18_str).unwrap();
            sites.extend(sites_18);
            let mut position_num = 20.0;
            for mut site_info in sites {
                site_info.position = Some(position_num);
                insert_into(site).values(&site_info).execute(&mut conn).unwrap();
                position_num += 20.0;
            }
        }
    }

    #[command]
    pub fn select_site() -> Vec<Site> {
        let mut conn = get_once_db_conn();
        let sites = site.select(Site::as_select()).order(position.asc()).load::<Site>(&mut conn).unwrap();
        sites
    }
    
    #[command]
    pub fn get_site_by_key(key: &str) -> Option<Site> {
        let mut conn = get_once_db_conn();
        let site_info = site.filter(site_key.is(key))
            .select(Site::as_select()).first::<Site>(&mut conn);
        match site_info {
            Ok(si) => Some(si),
            Err(_) => None,
        }
    }

    #[command]
    pub fn save_site(mut site_info: Site) {
        let mut conn = get_once_db_conn();
        if site_info.site_key == "" {
            site_info.site_key = utils::get_pinyin_first_letter(&site_info.site_key)
        }
        match site_info.id {
            Some(site_info_id) => {
                let _result = update(site)
                .set((site_key.eq(site_info.site_key), site_name.eq(site_info.site_name), api.eq(site_info.api), site_group.eq(site_info.site_group), 
                    is_active.eq(site_info.is_active), status.eq(site_info.status),  position.eq(site_info.position), 
                    is_reverse_order.eq(site_info.is_reverse_order)))
                .filter(id.eq(site_info_id)).execute(&mut conn);
            }
            _ => {
                let position_max = site.select(max(position)).first::<Option<f64>>(&mut conn).unwrap().unwrap_or(0.00);
                site_info.position = Some(position_max + 10.0);
                insert_into(site).values(&site_info).execute(&mut conn).unwrap();
            }
        }
    }
    
    #[command]
    pub fn insert_sites(sites: Vec<Site>) {
        for mut site_info in sites {
            site_info.id = None;
            save_site(site_info);
        }
    }

    #[command]
    pub fn del_site(site_id: i32) {
        let mut conn = get_once_db_conn();
        delete(site.filter(id.eq(&site_id))).execute(&mut conn).unwrap();
    }

    #[command]
    pub fn reset_site() {
        let mut conn = get_once_db_conn();
        delete(site).execute(&mut conn).unwrap();
        check_init_site();
    }

}
