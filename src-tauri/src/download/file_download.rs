use colored::Colorize;
use crossbeam::queue::SegQueue;
use m3u8_rs::{MediaPlaylist, Playlist};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use url::Url;
use std::fs::OpenOptions;
use std::io::prelude::*;
use lazy_static::lazy_static;
use std::{
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    process::Command,
    sync::{atomic::AtomicI32, atomic::Ordering, Arc, Mutex},
    thread,
    time::Duration,
};
use tokio::{
    fs::{remove_file, File},
    sync::Semaphore,
};
use tokio::{io::AsyncWriteExt, sync::mpsc};

use log::{error, info};
use tungstenite::{accept, handshake::HandshakeRole, Error, HandshakeError, Message, Result};

use crate::{download::{
    m3u8_encrypt_key::{KeyType, M3u8EncryptKey},
    util::download_request,
}, data::download_info::{cmd::{get_download_not_end, update_download_info_by_context, update_download_count}, get_download_movie_path}};
use crate::utils;
use crate::data::download_info::DownloadInfo;

lazy_static! {
    pub static ref DOWNLOAD_QUEUE: Mutex<SegQueue<DownloadInfo>> = Mutex::new(SegQueue::new());
}

pub fn init_download_queue() {
    let download_info_list = get_download_not_end();
    for download_info1 in download_info_list {
        let queue = DOWNLOAD_QUEUE.lock().unwrap();
        queue.push(download_info1);
    }
}



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
    //消息类型 progress 进度切换 statusChange
    pub mes_type: String,
    pub download_status: String,
    pub index_path: PathBuf,
    pub json_path: PathBuf,
    pub ts_path: PathBuf,
}

impl DownloadInfoContext {
    pub fn new(download_info: &mut DownloadInfo) -> Self {
        let movie_name = download_info.movie_name.clone();
        let sub_title_name = &download_info.sub_title_name;

        let movie_path = get_download_movie_path(&movie_name, sub_title_name);

        utils::mkdir(&movie_path);

        let index_path = movie_path.join(sub_title_name.to_owned() + ".txt");
        let json_path = movie_path.join(sub_title_name.to_owned() + ".json");
        let ts_path = movie_path.join("ts");
        Self {
            id: download_info.id.unwrap(),
            url: Url::parse(&download_info.url).unwrap(),
            movie_name: download_info.movie_name.clone(),
            sub_title_name: download_info.sub_title_name.clone(),
            status: download_info.status.clone(),
            mes_type: "statusChange".into(),
            download_status: download_info.download_status.clone(),
            count: download_info.count,
            download_count: download_info.download_count,
            index_path,
            json_path,
            ts_path,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadRequest {
    pub id: String,
    pub downloadInfo: DownloadInfo,
}

pub async fn init() {
    let server = TcpListener::bind("127.0.0.1:8000").unwrap();
    for stream in server.incoming() {
        thread::spawn(move || match stream {
            Ok(stream) => {
                if let Err(err) = handle_client(stream) {
                    match err {
                        Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                        e => error!("test: {}", e),
                    }
                }
            }
            Err(e) => error!("Error accepting stream: {}", e),
        });
    }
}

fn must_not_block<Role: HandshakeRole>(err: HandshakeError<Role>) -> Error {
    match err {
        HandshakeError::Interrupted(_) => panic!("Bug: blocking socket would block"),
        HandshakeError::Failure(f) => f,
    }
}

#[tokio::main]
async fn handle_client(stream: TcpStream) -> Result<()> {
    let mut socket = accept(stream).map_err(must_not_block)?;
    info!("Running test");
    loop {
        match socket.read()? {
            msg @ Message::Text(_) | msg @ Message::Binary(_) => {
                let mut request =
                    serde_json::from_str::<DownloadRequest>(&msg.clone().into_text().unwrap())
                        .unwrap();
                let mut download_info_context = DownloadInfoContext::new(&mut request.downloadInfo);

                match &download_info_context.status[..] {
                    "parseSource" => {
                        let content = download_request(&download_info_context.url).await;
                        match content {
                            Ok(content1) => {

                                let media_play_list: MediaPlaylist;

                                match m3u8_rs::parse_playlist_res(&content1).expect("Invalid .m3u8 format") {
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
                                        download_info_context.url =
                                            download_info_context.url.join(&stream.uri).unwrap();
        
                                        let play_list = m3u8_rs::parse_playlist_res(
                                            &download_request(&download_info_context.url).await.unwrap(),
                                        )
                                        .expect("play list not found");
                                        match play_list {
                                            Playlist::MediaPlaylist(media_list) => {
                                                media_play_list = media_list
                                            }
                                            _ => panic!("media play list not found"),
                                        }
                                    }
                                    Playlist::MediaPlaylist(media_list) => media_play_list = media_list,
                                }
        
                                let count = media_play_list.segments.len();
                                download_info_context.count = count as i32;
        
                                let mut download_source_info = DownloadSourceInfo::new();
                                download_source_info.id = download_info_context.id;
        
                                utils::mkdir(&download_info_context.ts_path);
        
                                let index = download_info_context.index_path.clone();
                                if utils::exists(&index) {
                                    remove_file(&index).await.unwrap();
                                }
        
                                utils::create_file(&index).unwrap();
        
                                let mut download_list: Vec<DownloadInfoDetail> = Vec::new();
        
                                for (i, segment) in media_play_list.segments.iter().enumerate() {
                                    let file_name = download_info_context
                                        .ts_path
                                        .join(Path::new(&segment.uri).file_name().unwrap());
                                    let file_name_str = utils::get_path_name(file_name.clone());
                                    if let Some(k) = &segment.key {
                                        let base_key_url = download_info_context.url.clone();
                                        download_source_info.m3u8_encrypt_key =
                                            M3u8EncryptKey::from_key(base_key_url, k).await.unwrap();
                                    }
        
                                    let mut index_file =
                                        OpenOptions::new().append(true).open(index.clone()).unwrap();
                                    let s = format!("{} {} {}", "file", file_name_str, "\n");
                                    index_file.write(s.as_bytes()).expect("写入文件异常");
        
                                    // 判断文件碎片是否已下载
                                    if file_name.is_file() {
                                        if File::open(&file_name).await?.metadata().await?.len() != 0 {
                                            println!(
                                                "Pass {}",
                                                &file_name.file_name().unwrap().to_str().unwrap()
                                            );
                                            continue;
                                        }
                                    }
        
                                    let base_download_url = download_info_context.url.clone();
                                    let url = base_download_url.join(&segment.uri).unwrap();
                                    download_list.push(DownloadInfoDetail {
                                        id: i,
                                        url,
                                        file_name: file_name.into_os_string().into_string().unwrap(),
                                        success: false,
                                    });
                                }
                                download_source_info.download_info_list = download_list;
        
                                if let Ok(v) = serde_json::to_string_pretty(&download_source_info) {
                                    std::fs::write(download_info_context.json_path.clone(), v).unwrap();
                                }
        
                                download_info_context.status = "downloadSlice".to_string();
                                download_info_context.download_status = "downloading".to_string();
        
                                let download_info_context1 = download_info_context.clone();
                                update_download_info_by_context(download_info_context);
                                socket.send(tungstenite::Message::Text(
                                    serde_json::to_string(&download_info_context1).unwrap(),
                                ))?
                            }
                            Err(_) => {
                                println!("请求响应数据错误");
                                socket.send(tungstenite::Message::Text(
                                    serde_json::to_string(&download_info_context).unwrap(),
                                ))?
                            },
                        }
                    }
                    "downloadSlice" => {
                        let download_count = AtomicI32::new(download_info_context.download_count);
                        let queue: SegQueue<DownloadInfoQueueDetail> = SegQueue::new();

                        let semaphore = Arc::new(Semaphore::new(6));

                        let v = std::fs::read_to_string(download_info_context.json_path.clone())
                            .unwrap();
                        let mut download_source_info =
                            serde_json::from_str::<DownloadSourceInfo>(&v).unwrap();
                        let download_info_list = download_source_info.download_info_list.clone();
                        for download_info in download_info_list {
                            queue.push(DownloadInfoQueueDetail {
                                id: download_info.id,
                                url: download_info.url,
                                file_name: download_info.file_name,
                                m3u8_encrypt_key: Arc::new(
                                    download_source_info.m3u8_encrypt_key.clone(),
                                ),
                            });
                        }

                        let (tx, mut rx): (
                            mpsc::Sender<Option<DownloadInfoDetail>>,
                            mpsc::Receiver<Option<DownloadInfoDetail>>,
                        ) = mpsc::channel(100);

                        while queue.len() > 0 {
                            let detail = queue.pop().unwrap();
                            let semaphore1 = semaphore.clone();
                            let tx1 = tx.clone();
                            tokio::spawn(async move {
                                #[allow(unused_variables)]
                                let p = semaphore1.acquire().await;
                                let resp_data = reqwest::get(detail.url.as_str()).await;
                                let mut fail = false;
                                match resp_data {
                                    Ok(rp) => {
                                        if rp.status() == StatusCode::OK {
                                            let mut data = rp.bytes().await.unwrap().to_vec();

                                            if !matches!(&detail.m3u8_encrypt_key.ty, KeyType::None)
                                            {
                                                match detail.m3u8_encrypt_key.decode(&mut data) {
                                                    Some(data1) => data = data1,
                                                    _ => {}
                                                };
                                            }

                                            let mut file =
                                                File::create(&detail.file_name).await.unwrap();
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
                                    tx1.send(Some(DownloadInfoDetail {
                                        id: detail.id.to_owned(),
                                        url: detail.url,
                                        file_name: detail.file_name.to_owned(),
                                        success: false,
                                    }))
                                    .await
                                    .unwrap();
                                }
                            });
                        }

                        drop(tx);

                        download_source_info.download_info_list.clear();
                        while let Some(res) = rx.recv().await {
                            match res {
                                Some(p) => download_source_info.download_info_list.push(p),
                                None => {
                                    let download_count1 =
                                        download_count.fetch_add(1, Ordering::Relaxed);
                                    update_download_count(
                                        download_info_context.id,
                                        download_count1,
                                    );
                                    let mut download_info_context2 = download_info_context.clone();
                                    download_info_context2.mes_type = "progress".to_string();
                                    download_info_context2.download_count = download_count1;
                                    socket.send(tungstenite::Message::Text(
                                        serde_json::to_string(&download_info_context2).unwrap(),
                                    ))?
                                }
                            }
                        }

                        if let Ok(v) = serde_json::to_string_pretty(&download_source_info) {
                            std::fs::write(download_info_context.json_path.clone(), v).unwrap();
                        }
                        drop(rx);

                        download_info_context.status = "checkSouce".to_string();
                        download_info_context.download_count =
                            download_count.load(Ordering::Relaxed);

                        let download_info_context1 = download_info_context.clone();
                        update_download_info_by_context(download_info_context);

                        socket.send(tungstenite::Message::Text(
                            serde_json::to_string(&download_info_context1).unwrap(),
                        ))?
                    }
                    "checkSouce" => {
                        let mut downloaded = true;
                        
                        let v = std::fs::read_to_string(download_info_context.json_path.clone())
                            .unwrap();
                        let download_source_info =
                            serde_json::from_str::<DownloadSourceInfo>(&v).unwrap();
                        if download_source_info.download_info_list.len() != 0 {
                            downloaded = false;
                            thread::sleep(Duration::from_secs(10));
                        }
                        
                        if downloaded {
                            download_info_context.status = "merger".to_string();
                            download_info_context.download_status = "downloading".to_string();
                        } else {
                            download_info_context.status = "downloadSlice".to_string();
                            download_info_context.download_status = "downloading".to_string();
                        }

                        let download_info_context1 = download_info_context.clone();
                        update_download_info_by_context(download_info_context);

                        socket.send(tungstenite::Message::Text(
                            serde_json::to_string(&download_info_context1).unwrap(),
                        ))?
                    }
                    "merger" => {
                        let index_str =
                            utils::get_path_name(download_info_context.index_path.clone());
                        let mv_str = index_str.replace("txt", "mp4");
                        File::create(Path::new(&mv_str)).await.unwrap();
                        let mut cmd = Command::new("ffmpeg");
                        let output = cmd
                            .args([
                                "-y",
                                "-f",
                                "concat",
                                "-safe",
                                "0",
                                "-i",
                                &index_str,
                                "-bsf:a",
                                "aac_adtstoasc",
                                "-c",
                                "copy",
                                &mv_str,
                            ])
                            .output()
                            .expect("视频转码异常!");

                        if output.status.success() {
                            // let mut index_path = PathBuf::from(index_str);
                            // index_path.pop();
                            // let ts_path = index_path.join("ts");
                            // remove_dir_all(ts_path).unwrap();
                            download_info_context.status = "downloadEnd".to_string();
                            download_info_context.download_status = "downloadSuccess".to_string();
                        } else {
                            download_info_context.download_status = "downloadFail".to_string();
                            let s = String::from_utf8_lossy(&output.stderr);
                            println!("视频转码失败:{:X?}", s);
                        }
                        let download_info_context1 = download_info_context.clone();
                        update_download_info_by_context(download_info_context);

                        socket.send(tungstenite::Message::Text(
                            serde_json::to_string(&download_info_context1).unwrap(),
                        ))?
                    }
                    _ => {
                        socket.send(tungstenite::Message::Text("不支持的操作".to_string()))?
                    }
                }
            }
            Message::Ping(_) | Message::Pong(_) | Message::Close(_) | Message::Frame(_) => {}
        }
    }
}

pub mod cmd {
    use tauri::command;
    use crate::data::download_info::DownloadInfo;
    use super::DOWNLOAD_QUEUE;

    #[command]
    pub fn get_download_info_by_queue() -> Option<DownloadInfo> {
        let queue = DOWNLOAD_QUEUE.lock().unwrap();
        queue.pop()
    }

    #[command]
    pub fn retry_download(download: DownloadInfo) {
        let queue = DOWNLOAD_QUEUE.lock().unwrap();
        queue.push(download);
    }
}
