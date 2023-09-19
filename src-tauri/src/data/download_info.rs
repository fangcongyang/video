use crate::download::m3u8::DownloadInfoContext;

use super::data_source_con::*;
use serde::{Deserialize, Serialize};
use std::{
    path::PathBuf,
    thread,
    time::Duration,
};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadInfo {
    pub id: i32,
    pub movieName: String,
    pub url: String,
    pub subTitleName: String,
    // parseSource 解析资源 downloadSlice 下载切片  checkSouce 检查资源完整性 merger 合并资源  downloadEnd 下载完成
    pub status: String,
    pub downloadCount: i32,
    pub count: i32,
    pub parentId: String,
    // wait 等待下载 downloading 下载中 downloadFail 下载失败 downloadSuccess 下载成功
    pub downloadStatus: String,
}

#[tokio::main]
pub async fn init() {
    // 开启一个线程每秒扫描下载进度
    thread::spawn(move || loop {
        let conn = get_db_conn();
        let mut stmt = conn.prepare("SELECT * FROM download_info WHERE status != 'downloadEnd' LIMIT 2").unwrap();
        let download_infos = stmt.query_map([], |row| {
                Ok(DownloadInfo {
                    id: row.get(0)?,
                    movieName: row.get(1)?,
                    url: row.get(2)?,
                    subTitleName: row.get(3)?,
                    status: row.get(4)?,
                    downloadCount: row.get(5)?,
                    count: row.get(6)?,
                    parentId: row.get(7)?,
                    downloadStatus: row.get(8)?,
                })
            })
            .unwrap();

        let download_infos: Vec<DownloadInfo> = download_infos
            .map(|download_info| download_info.unwrap())
            .collect();

        if !download_infos.is_empty() {
            let mut download_info2;

            let mut job_handles = Vec::new();
            for download_info in download_infos {
                download_info2 = download_info.clone();
                let job_handle = thread::spawn(move || {
                    let download_info_context = DownloadInfoContext::new(&mut download_info2);
                    download_info_context.download();
                });
                job_handles.push(job_handle);
            }

            for job_handle in job_handles {
                job_handle.join().unwrap();
            }
        }
        thread::sleep(Duration::from_secs(1));
    });
}

pub mod cmd {
    use crate::conf::AppConf;

    use super::*;
    use rusqlite::named_params;
    use tauri::command;

    #[command]
    pub fn select_download_info() -> Vec<DownloadInfo> {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();

        let mut stmt = conn.prepare("SELECT * FROM download_info").unwrap();
        let download_infos = stmt
            .query_map([], |row| {
                Ok(DownloadInfo {
                    id: row.get(0)?,
                    movieName: row.get(1)?,
                    url: row.get(2)?,
                    subTitleName: row.get(3)?,
                    status: row.get(4)?,
                    downloadCount: row.get(5)?,
                    count: row.get(6)?,
                    parentId: row.get(7)?,
                    downloadStatus: row.get(8)?,
                })
            })
            .unwrap();
        let download_infos: Vec<DownloadInfo> = download_infos
            .map(|download_info| download_info.unwrap())
            .collect();
        download_infos
    }

    #[command]
    pub fn insert_download_infos(download_infos: Vec<DownloadInfo>) {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        for download_info in download_infos {
            conn.execute(
                "INSERT INTO download_info (movie_name, url, sub_title_name, status, download_count, count, parent_id, download_status) 
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                (&download_info.movieName, &download_info.url, &download_info.subTitleName, &download_info.status, &download_info.downloadCount,
                    &download_info.count, &download_info.parentId, &download_info.downloadStatus),
            ).unwrap();
        }
    }

    #[command]
    pub fn get_download_by_id(id: i32) -> DownloadInfo {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();

        let mut download_info = conn
            .query_row(
                "SELECT * FROM download_info where id = :id",
                named_params! { ":id": id },
                |row| {
                    Ok(DownloadInfo {
                        id: row.get(0)?,
                        movieName: row.get(1)?,
                        url: row.get(2)?,
                        subTitleName: row.get(3)?,
                        status: row.get(4)?,
                        downloadCount: row.get(5)?,
                        count: row.get(6)?,
                        parentId: row.get(7)?,
                        downloadStatus: row.get(8)?,
                    })
                },
            )
            .unwrap();

        let app_conf = AppConf::read();
        let mut movie_path = PathBuf::from(&app_conf.systemConf.downloadSavePath);
        let movie_name = download_info.movieName.clone();
        movie_path = movie_path.join(&movie_name);
        let sub_title_name = &download_info.subTitleName;
        if sub_title_name != "" {
            movie_path = movie_path
                .join(&sub_title_name)
                .join(sub_title_name.to_owned() + ".mp4");
        } else {
            movie_path = movie_path.join(movie_name.to_owned() + ".mp4");
        }
        download_info.url = movie_path.into_os_string().into_string().unwrap();
        download_info
    }
}
