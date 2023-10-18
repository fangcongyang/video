use super::data_source_con::*;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, QueryableByName, Insertable)]
#[diesel(table_name = crate::schema::channel_group)]
pub struct ChannelGroup {
    id: Option<i32>,
    channel_name: String,
    channel_group_name: String,
    channel_active: String,
    channel_status: String,
    position: Option<f64>,
    channels: String
}

pub mod cmd {
    use super::*;
    use tauri::command;
    use diesel::{insert_into, delete, dsl::max};
    use crate::schema::channel_group::dsl::*;

    #[command]
    pub fn select_channel_group() -> Vec<ChannelGroup> {
        let mut conn = get_once_db_conn();
        let channel_groups = channel_group.select(ChannelGroup::as_select()).order_by(position.desc()).load::<ChannelGroup>(&mut conn).unwrap();
        channel_groups
    }

    #[command]
    pub fn save_channel_group(mut channel_group_info: ChannelGroup) {
        let mut conn = get_once_db_conn();
        match channel_group_info.id {
            Some(channel_group_info_id) => {
                let _result = diesel::update(channel_group)
                .set((channel_name.eq(channel_group_info.channel_name), channel_group_name.eq(channel_group_info.channel_group_name), 
                    channel_active.eq(channel_group_info.channel_active), channel_status.eq(channel_group_info.channel_status), 
                    position.eq(channel_group_info.position), channels.eq(channel_group_info.channels)))
                .filter(id.eq(channel_group_info_id)).execute(&mut conn);
            }
            _ => {
                let position_max = channel_group.select(max(position)).first::<Option<f64>>(&mut conn).unwrap().unwrap_or(0.00);
                channel_group_info.position = Some(position_max + 10.0);
                insert_into(channel_group).values(&channel_group_info).execute(&mut conn).unwrap();
            }
        }
    }

    #[command]
    pub fn save_channel_groups(channel_groups: Vec<ChannelGroup>) {
        for channel_group_info in channel_groups {
            save_channel_group(channel_group_info);
        }
    }

    #[command]
    pub fn del_channel_group(channel_group_id: i32) {
        let mut conn = get_once_db_conn();
        delete(channel_group.filter(id.eq(&channel_group_id))).execute(&mut conn).unwrap();
    }
    
    #[command]
    pub fn del_channel_groups(channel_group_ids: Vec<i32>) {
        let mut conn = get_once_db_conn();
        if channel_group_ids.len() == 0 {
            return;
        }
        delete(channel_group.filter(id.eq_any(&channel_group_ids))).execute(&mut conn).unwrap();
    }

}
