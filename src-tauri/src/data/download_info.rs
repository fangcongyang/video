use crate::utils;
use super::data_source_con::*;
use lru::LruCache;
use serde::{Serialize, Deserialize};
use std::{thread, time::Duration, sync::Mutex, fs, num::NonZeroUsize};
use lazy_static::lazy_static;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct DownloadInfo {
    pub id: i32,
    pub movieName: String,
    pub url: String,
    pub subTitleName: String,
    pub status: String,
    pub downloadCount: i32,
    pub count: i32,
    pub parentId: String,
}

pub struct DownloadInfoCache {
    lru: LruCache<i32, DownloadInfo>,
}

impl DownloadInfoCache {
    fn new(capacity: usize) -> DownloadInfoCache {
        DownloadInfoCache {
            lru: LruCache::new(NonZeroUsize::new(capacity).unwrap()),
        }
    }

    pub fn get_lru(&mut self) -> &mut LruCache<i32, DownloadInfo> {
        &mut self.lru
    }

    pub fn put(&mut self, id: i32, download_info: DownloadInfo) {
        self.lru.put(id, download_info);
    }

    pub fn get(&mut self, id: i32) -> Option<&mut DownloadInfo> {
        self.lru.get_mut(&id)
    }

    pub fn get_len(&mut self) -> usize {
        self.lru.len()
    }
}

lazy_static! {
    pub static ref DOWNLOAD_INFO_CACHE: Mutex<DownloadInfoCache> = Mutex::new(DownloadInfoCache::new(2));
}

pub fn init() {
    // 开启一个线程每秒扫描下载进度
    thread::spawn(move || {
        let mut binging = DOWNLOAD_INFO_CACHE.lock().unwrap();
        let mut path = utils::app_install_root();
        for (_key, download_info) in binging.get_lru().iter_mut() {
            path.pop();
            let movie_path = path.join("download").join(&download_info.movieName);
            let sub_title_name = &download_info.subTitleName;
            let ts_path;
            if sub_title_name != "" {
                ts_path = movie_path.join(&sub_title_name).join("ts");
            } else {
                ts_path = movie_path.join("ts");
            }
            let entries = fs::read_dir(ts_path).unwrap();
            download_info.downloadCount = entries.count() as i32;
            cmd::save_download_info(DownloadInfo{
                id: download_info.id,
                movieName: download_info.movieName.clone(),
                url: download_info.url.clone(),
                subTitleName: download_info.subTitleName.clone(),
                status: download_info.status.clone(),
                downloadCount: download_info.downloadCount,
                count: download_info.count,
                parentId: download_info.parentId.clone(),
            });
        }
        thread::sleep(Duration::from_secs(1));
    });
}

pub mod cmd {
    use crate::download::m3u8::download_file;

    use super::*;
    use tauri::command;

    #[command]
    pub fn select_download_info() -> Vec<DownloadInfo> {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        
        let mut stmt = conn.prepare("SELECT * FROM download_info").unwrap();
        let download_infos = stmt.query_map([], |row| {
            Ok(DownloadInfo {
                id: row.get(0)?,
                movieName: row.get(1)?,
                url: row.get(2)?,
                subTitleName: row.get(3)?,
                status: row.get(4)?,
                downloadCount: row.get(5)?,
                count: row.get(6)?,
                parentId: row.get(7)?
            })
        }).unwrap();
        let download_infos: Vec<DownloadInfo> = download_infos.map(|download_info| download_info.unwrap()).collect();
        download_infos
    }

    #[command]
    pub fn save_download_info(download_info: DownloadInfo) {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        if download_info.id == 0 {
            conn.execute(
                "INSERT INTO download_info (movie_name, url, sub_title_name, status, download_count, count, parent_id) 
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                (&download_info.movieName, &download_info.url, &download_info.subTitleName, &download_info.status, &download_info.downloadCount,
                    &download_info.count, &download_info.parentId),
            ).unwrap();
        } else {
            conn.execute(
                "UPDATE download_info SET status = ?1, download_count = ?2, count = ?3 WHERE id = ?4",
                (&download_info.status, &download_info.downloadCount, &download_info.count, &download_info.id),
            ).unwrap();
        }
    }

    #[command]
    pub fn download(download_info: DownloadInfo) {
        if DOWNLOAD_INFO_CACHE.lock().unwrap().get_len() == 2 {
            return;
        }
        let id = download_info.id.clone();
        DOWNLOAD_INFO_CACHE.lock().unwrap().put(download_info.id, download_info);
        thread::spawn(move || {
            download_file(id);
        });
    }

}
