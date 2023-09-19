use colored::Colorize;
use m3u8_rs::{MediaPlaylist, Playlist};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::{
    path::{Path, PathBuf},
    process::Command,
    sync::Arc,
};
use tokio::fs::{read_dir, remove_file};
use tokio::io::AsyncWriteExt;
use tokio::{
    fs::File,
    sync::Semaphore,
};
use url::Url;

use super::m3u8_encrypt_key::{KeyType, M3u8EncryptKey};
use super::queue::CustomQueue;
use super::util::download;
use crate::conf::AppConf;
use crate::data::data_source_con::{CACHE, DBNAME};
use crate::data::download_info::DownloadInfo;
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

pub struct DownloadInfoContext {
    pub id: i32,
    pub url: Url,
    pub count: i32,
    pub movie_name: String,
    pub sub_title_name: String,
    pub status: String,
    pub index_path: PathBuf,
    pub json_path: PathBuf,
    pub ts_path: PathBuf,
}

impl DownloadInfoContext {
    pub fn new(download_info: &mut DownloadInfo) -> Self {
        let app_conf = AppConf::read();
        let path = PathBuf::from(&app_conf.systemConf.downloadSavePath);

        let movie_name = download_info.movieName.clone();
        let movie_path = path.join(&download_info.movieName);

        utils::mkdir(&movie_path);

        let index_path;
        let sub_title_name = &download_info.subTitleName;
        let json_path;
        let ts_path;
        if sub_title_name != "" {
            index_path = movie_path
                .join(&sub_title_name)
                .join(sub_title_name.to_owned() + ".txt");
            json_path = movie_path
                .join(&sub_title_name)
                .join(sub_title_name.to_owned() + ".json");
            ts_path = movie_path.join(&sub_title_name).join("ts");
        } else {
            index_path = movie_path.join(movie_name.to_owned() + ".txt");
            json_path = movie_path.join(movie_name.to_owned() + ".json");
            ts_path = movie_path.join("ts");
        }
        Self {
            id: download_info.id,
            url: Url::parse(&download_info.url).unwrap(),
            movie_name: download_info.movieName.clone(),
            sub_title_name: download_info.subTitleName.clone(),
            status: download_info.status.clone(),
            count: download_info.count,
            index_path,
            json_path,
            ts_path,
        }
    }

    #[tokio::main]
    pub async fn download(self) {
        match &self.status[..] {
            "parseSource" => {
                self.get_download_source_info().await;
            }
            "checkSouce" => {
                #[allow(unused_variables)]
                let r = self.check_downloaded().await;
            }
            "downloadSlice" => {
                #[allow(unused_variables)]
                let r = self.queue_down().await;
            }
            "merger" => {
                #[allow(unused_variables)]
                let r = self.merger().await;
            }
            _ => {}
        }
    }

    pub async fn get_download_source_info(mut self) {
        let content = download(&self.url).await.unwrap();

        match m3u8_rs::parse_playlist_res(&content).expect("Invalid .m3u8 format") {
            Playlist::MasterPlaylist(master) => {
                for (i, vs) in master.variants.iter().enumerate() {
                    println!(
                        "{index}: \n {stream:#?}",
                        index = format!("#{}", i).blue(),
                        stream = vs
                    )
                }

                let stream = master
                    .variants
                    .get(0)
                    .expect(&"Please select a valid number!".red());
                self.url = self.url.join(&stream.uri).unwrap();

                let play_list = m3u8_rs::parse_playlist_res(&download(&self.url).await.unwrap())
                    .expect("play list not found");
                match play_list {
                    Playlist::MediaPlaylist(media_list) => {
                        self.parse_media_list(media_list).await.unwrap()
                    }
                    _ => panic!("media play list not found"),
                }
            }
            Playlist::MediaPlaylist(list) => {
                self.parse_media_list(list).await.unwrap();
            }
        }
    }

    pub async fn parse_media_list(mut self, play_list: MediaPlaylist) -> anyhow::Result<()> {
        let count = play_list.segments.len();
        self.count = count as i32;

        let mut download_source_info = DownloadSourceInfo::new();
        download_source_info.id = self.id;

        utils::mkdir(&self.ts_path);

        let index = self.index_path.clone();
        if utils::exists(&index) {
            remove_file(&index).await.unwrap();
        }

        utils::create_file(&index).unwrap();

        let mut download_list: Vec<DownloadInfoDetail> = Vec::new();

        for (i, segment) in play_list.segments.iter().enumerate() {
            let file_name = self
                .ts_path
                .join(Path::new(&segment.uri).file_name().unwrap());
            let file_name_str = utils::get_path_name(file_name.clone());
            if let Some(k) = &segment.key {
                let base_key_url = self.url.clone();
                download_source_info.m3u8_encrypt_key =
                    M3u8EncryptKey::from_key(base_key_url, k).await?
            }

            let mut index_file = OpenOptions::new().append(true).open(index.clone()).unwrap();
            let s = format!("{} {} {}", "file", file_name_str, "\n");
            index_file.write(s.as_bytes()).expect("写入文件异常");

            // 判断文件碎片是否已下载
            if file_name.is_file() {
                if File::open(&file_name).await?.metadata().await?.len() != 0 {
                    println!("Pass {}", &file_name.file_name().unwrap().to_str().unwrap());
                    continue;
                }
            }

            let base_download_url = self.url.clone();
            let url = base_download_url.join(&segment.uri)?;
            download_list.push(DownloadInfoDetail {
                id: i,
                url,
                file_name: file_name.into_os_string().into_string().unwrap(),
                success: false,
            });
        }
        download_source_info.download_info_list = download_list;

        if let Ok(v) = serde_json::to_string_pretty(&download_source_info) {
            std::fs::write(self.json_path.clone(), v).unwrap();
        }

        update_download_info(
            self.id,
            "downloadSlice".to_string(),
            self.count.clone(),
            "downloading".to_string(),
        );

        Ok(())
    }

    pub async fn queue_down(self) -> anyhow::Result<()> {
        let mut queue: CustomQueue<DownloadInfoQueueDetail> = CustomQueue::new();

        let semaphore = Arc::new(Semaphore::new(6));

        let v = std::fs::read_to_string(self.json_path.clone()).unwrap();
        let mut download_source_info = serde_json::from_str::<DownloadSourceInfo>(&v).unwrap();
        let download_info_list = download_source_info.download_info_list.clone();
        for download_info in download_info_list {
            queue.enqueue(DownloadInfoQueueDetail {
                id: download_info.id,
                url: download_info.url,
                file_name: download_info.file_name,
                m3u8_encrypt_key: Arc::new(
                    download_source_info.m3u8_encrypt_key.clone(),
                ),
            });
        }

        let (tx, mut rx):(mpsc::Sender<Option<DownloadInfoDetail>>, mpsc::Receiver<Option<DownloadInfoDetail>>) 
        = mpsc::channel(100);

        while queue.size() > 0 {
            let detail = queue.dequeue().unwrap();
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let tx1 = tx.clone();
            tokio::spawn(async move {
                let resp_data = reqwest::get(detail.url.as_str()).await;
                let mut fail = false;
                match resp_data {
                    Ok(rp) => {
                        if rp.status() == StatusCode::OK {
                            let mut data = rp.bytes().await.unwrap().to_vec();

                            if !matches!(&detail.m3u8_encrypt_key.ty, KeyType::None) {
                                match detail.m3u8_encrypt_key.decode(&mut data) {
                                    Some(data1) => data = data1,
                                    _ => {}
                                };
                            }

                            let mut file = File::create(&detail.file_name).await.unwrap();
                            file.write_all(&data).await.unwrap();
                            tx1.send(None).await.unwrap();
                        } else {
                            fail = true;
                        }
                    }
                    Err(_e) => {
                        fail = true;
                    }
                }

                if fail {
                    tx1.send(Some(DownloadInfoDetail{
                        id: detail.id.to_owned(),
                        url: detail.url,
                        file_name: detail.file_name.to_owned(),
                        success: false,
                    })).await.unwrap();
                }
                drop(permit);
            });
        }

        drop(tx);

        download_source_info.download_info_list.clear();

        while let Some(res) = rx.recv().await {
            match res {
                Some(p) => download_source_info.download_info_list.push(p),
                None => (),
            }
        }

        if let Ok(v) = serde_json::to_string_pretty(&download_source_info) {
            std::fs::write(self.json_path.clone(), v).unwrap();
        }
        drop(rx);

        update_download_info(
            self.id,
            "checkSouce".to_string(),
            self.count.clone(),
            "downloading".to_string(),
        );

        Ok(())
    }

    pub async fn check_downloaded(self) -> anyhow::Result<()> {
        let mut entries = read_dir(self.ts_path).await.unwrap();
        while let Some(entry) = entries.next_entry().await? {
            if File::open(&entry.path()).await?.metadata().await?.len() == 0 {
                update_download_info(
                    self.id,
                    "downloadSlice".to_string(),
                    self.count.clone(),
                    "downloading".to_string(),
                );
                return Ok(());
            }
        }

        update_download_info(
            self.id,
            "merger".to_string(),
            self.count.clone(),
            "downloading".to_string(),
        );
        Ok(())
    }

    pub async fn merger(self) -> anyhow::Result<()> {
        let index_str = utils::get_path_name(self.index_path.clone());
        let mv_str = index_str.replace("txt", "mp4");
        File::create(Path::new(&mv_str)).await.unwrap();
        let app_conf = AppConf::read();
        println!("视频转码成功:{:X?}", app_conf.systemConf.ffmpegPath);
        let args = format!(
            "{} -y -f concat -safe 0 -i {} -bsf:a aac_adtstoasc -c copy {}",
            &app_conf.systemConf.ffmpegPath, index_str, mv_str
        );
        let mut cmd = Command::new("cmd");
        let output = cmd.args(["/C", &args]).output().expect("视频转码异常!");

        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);
            update_download_info(
                self.id,
                "downloadEnd".to_string(),
                self.count.clone(),
                "downloadSuccess".to_string(),
            );
            println!("视频转码成功:{:X?}", s);
        } else {
            let s = String::from_utf8_lossy(&output.stderr);
            update_download_info(
                self.id,
                "merger".to_string(),
                self.count.clone(),
                "downloadFail".to_string(),
            );
            println!("视频转码失败:{:X?}", s);
        }

        Ok(())
    }
}

pub fn update_download_info(
    id: i32,
    status: String,
    count: i32,
    download_status: String,
) {
    let mut binding = CACHE.lock().unwrap();
    let conn = binding.get(DBNAME.into()).unwrap();

    conn.execute(
        "UPDATE download_info SET status = ?1, count = ?2, download_status = ?3 WHERE id = ?4",
        (&status, &count, &download_status, &id),
    ).unwrap();
}
