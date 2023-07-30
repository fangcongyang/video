use super::data_source_con::*;
use serde::{Serialize, Deserialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelGroup {
    id: i32,
    name: String,
    channelGroup: String,
    active: String,
    status: String,
    hasChildren: String,
    channels: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    id: String,
    url: String,
    active: String,
    status: String
}

pub mod cmd {
    use super::*;
    use rusqlite::params;
    use tauri::command;

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ChannelGroupDto {
        id: i32,
        name: String,
        channelGroup: String,
        active: String,
        status: String,
        hasChildren: String,
        channels: Vec<Channel>,
    }

    #[command]
    pub fn select_channel_group() -> Vec<ChannelGroup> {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        
        let mut stmt = conn.prepare("SELECT * FROM channel_group").unwrap();
        let channel_groups = stmt.query_map([], |row| {
            Ok(ChannelGroup {
                id: row.get(0)?,
                name: row.get(1)?,
                channelGroup: row.get(2)?,
                active: row.get(3)?,
                status: row.get(4)?,
                hasChildren: row.get(5)?,
                channels: row.get(6)?,
            })
        }).unwrap();
        
        let channel_group_list: Vec<ChannelGroup> = channel_groups.map(|channel_group| channel_group.unwrap()).collect();
        channel_group_list
    }

    #[command]
    pub fn save_channel_group(channel_group: ChannelGroup) {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        if channel_group.id == 0 {
            conn.execute(
                "INSERT INTO channel_group (name, channel_group, active, status, has_children, channels) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                (&channel_group.name, &channel_group.channelGroup, &channel_group.active, &channel_group.status, &channel_group.hasChildren,
                    &channel_group.channels),
            ).unwrap();
        } else {
            conn.execute(
                "UPDATE channel_group SET name = ?1, active = ?2, status = ?3, has_children = ?4, channels = ?5  WHERE id = ?6",
                (&channel_group.name, &channel_group.active, &channel_group.status, &channel_group.hasChildren, 
                    &channel_group.channels, &channel_group.id, ),
            ).unwrap();
        }
    }

    #[command]
    pub fn save_channel_groups(channel_groups: Vec<ChannelGroupDto>) {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        for channel_group in channel_groups {
            match Some(channel_group.id) {
                Some(_i) => conn.execute(
                    "INSERT INTO channel_group (name, channel_group, active, status, has_children, channels) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                    (&channel_group.name, &channel_group.channelGroup, &channel_group.active, &channel_group.status, &channel_group.hasChildren,
                        serde_json::to_string(&channel_group.channels).unwrap()),
                ).unwrap(),
                None => conn.execute(
                    "UPDATE channel_group SET name = ?1, active = ?2, status = ?3, has_children = ?4, channels = ?5  WHERE id = ?6",
                    (&channel_group.name, &channel_group.active, &channel_group.status, &channel_group.hasChildren, 
                        serde_json::to_string(&channel_group.channels).unwrap(), &channel_group.id, ),
                ).unwrap(),
            };
        }
    }

    #[command]
    pub fn del_channel_group(id: i32) {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        conn.execute("DELETE FROM channel_group WHERE id = ?1", params![&id]).unwrap();
    }

}
