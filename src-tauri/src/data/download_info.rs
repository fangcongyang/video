use crate::conf::AppConf;

use super::data_source_con::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use diesel::prelude::*;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, QueryableByName, Insertable)]
#[diesel(table_name = crate::schema::download_info)]
pub struct DownloadInfo {
    pub id: Option<i32>,
    #[diesel(column_name = "movie_name")]
    pub movieName: String,
    pub url: String,
    #[diesel(column_name = "sub_title_name")]
    pub subTitleName: String,
    // parseSource 解析资源 downloadSlice 下载切片  checkSouce 检查资源完整性 merger 合并资源  downloadEnd 下载完成
    pub status: String,
    #[diesel(column_name = "download_count")]
    pub downloadCount: i32,
    pub count: i32,
    // wait 等待下载 downloading 下载中 downloadFail 下载失败 downloadSuccess 下载成功
    #[diesel(column_name = "download_status")]
    pub downloadStatus: String,
}

pub fn get_download_movie_path(movie_name: &str, sub_title_name: &str) -> PathBuf {
    let app_conf = AppConf::read();
    let mut movie_path = PathBuf::from(&app_conf.systemConf.downloadSavePath);
    movie_path = movie_path.join(&movie_name);
    movie_path = movie_path.join(&sub_title_name);
    movie_path
}

pub mod cmd {

    use super::*;
    use diesel::{insert_into, delete, result::Error};
    use tauri::command;
    use std::fs;
    use crate::{schema::download_info::dsl::*, download::file_download::DOWNLOAD_QUEUE};

    #[command]
    pub fn select_download_info() -> Vec<DownloadInfo> {
        let mut conn = get_once_db_conn();
        let results = download_info.select(DownloadInfo::as_select()).load::<DownloadInfo>(&mut conn).unwrap();
        results
    }

    #[command]
    pub fn insert_download_infos(download_info_list: Vec<DownloadInfo>) {
        let mut conn = get_once_db_conn();
        let download_infos = conn.transaction::<_, Error, _>(|conn| {
            let inserted_count = insert_into(download_info)
                .values(&download_info_list)
                .execute(conn)?;

            Ok(download_info
                .order(id.desc())
                .limit(inserted_count as i64)
                .load(conn)?
                .into_iter()
                .rev()
                .collect::<Vec<_>>())
        }).unwrap();
        let queue = DOWNLOAD_QUEUE.lock().unwrap();
        for di in download_infos {
            queue.push(di);
        }
    }

    #[command]
    pub fn get_download_by_id(download_id: i32) -> DownloadInfo {
        let mut conn = get_once_db_conn();
        let mut download = download_info.filter(id.is(download_id))
            .select(DownloadInfo::as_select()).first::<DownloadInfo>(&mut conn).unwrap();
        let sub_title_name1 = &download.subTitleName;
        let mut movie_path = super::get_download_movie_path(&download.movieName.clone(), sub_title_name1);
        movie_path = movie_path
            .join(sub_title_name1.to_owned() + ".mp4");
        download.url = movie_path.into_os_string().into_string().unwrap();
        download
    }

    #[command]
    pub fn del_download_info(download: DownloadInfo) {
        let mut conn = get_once_db_conn();
        fs::remove_dir_all(super::get_download_movie_path(&download.movieName, &download.subTitleName)).unwrap();
        delete(download_info.filter(id.eq(&download.id))).execute(&mut conn).unwrap();
    }
    
    pub fn get_download_not_end() -> Vec<DownloadInfo> {
        let mut conn = get_once_db_conn();
        let download_info_list = download_info.filter(status.is_not("downloadEnd"))
            .select(DownloadInfo::as_select()).load(&mut conn).unwrap();
        download_info_list
    }
}
