use serde::{Deserialize, Serialize};
use std::{
    path::PathBuf,
    sync::Arc,
};
use url::Url;

use super::m3u8_encrypt_key::M3u8EncryptKey;
use crate::data::download_info::{DownloadInfo, get_download_movie_path};
use crate::utils;

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadSourceInfo {
    pub id: i32,
    pub m3u8_encrypt_key: M3u8EncryptKey,
    pub download_info_list: Vec<DownloadInfoDetail>,
}

impl DownloadSourceInfo {
    pub fn new() -> Self {
        Self {
            id: 0,
            m3u8_encrypt_key: M3u8EncryptKey::default(),
            download_info_list: [].to_vec(),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadInfoDetail {
    pub id: usize,
    pub url: Url,
    pub file_name: String,
    pub success: bool,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct DownloadInfoQueueDetail {
    pub id: usize,
    pub url: Url,
    pub file_name: String,
    pub m3u8_encrypt_key: Arc<M3u8EncryptKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadInfoContext {
    pub id: i32,
    pub url: Url,
    pub count: i32,
    pub download_count: i32,
    pub movie_name: String,
    pub sub_title_name: String,
    pub status: String,
    pub download_status: String,
    pub index_path: PathBuf,
    pub json_path: PathBuf,
    pub ts_path: PathBuf,
}

impl DownloadInfoContext {
    pub fn new(download_info: &mut DownloadInfo) -> Self {
        let movie_name = download_info.movieName.clone();
        let sub_title_name = &download_info.subTitleName;

        let movie_path = get_download_movie_path(&movie_name, sub_title_name);

        utils::mkdir(&movie_path);

        let index_path = movie_path.join(sub_title_name.to_owned() + ".txt");
        let json_path = movie_path.join(sub_title_name.to_owned() + ".json");
        let ts_path = movie_path.join("ts");
        Self {
            id: download_info.id,
            url: Url::parse(&download_info.url).unwrap(),
            movie_name: download_info.movieName.clone(),
            sub_title_name: download_info.subTitleName.clone(),
            status: download_info.status.clone(),
            download_status: download_info.downloadStatus.clone(),
            count: download_info.count,
            download_count: download_info.downloadCount,
            index_path,
            json_path,
            ts_path,
        }
    }
}

