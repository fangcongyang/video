use futures::stream::FuturesUnordered;
use m3u8_rs::{Key as m3u8Key, Playlist, MediaPlaylist, KeyMethod};
use reqwest::StatusCode;
use reqwest::header::{HeaderMap, HeaderValue, HeaderName};
use tokio::{sync::Semaphore, fs::File};
use tokio::io::AsyncWriteExt;
use std::{path::{Path, PathBuf}, sync::{Mutex, Arc}, time::Duration, process::Command, thread};
use std::io::{Cursor, Read};
use std::fs::OpenOptions;
use std::io::prelude::*;
use url::Url;
use colored::Colorize;
use aesstream::AesReader;

use crate::data::download_info::{DOWNLOAD_INFO_CACHE, DownloadInfo};
use crate::utils;
use super::queue::CustomQueue;

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct DownloadInfoDetail {
    pub id: usize,
    pub url: Url,
    pub m3u8_encrypt_key: Arc<M3u8EncryptKey>,
    pub semaphore: Arc<Semaphore>,
    pub file_name: PathBuf,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum KeyType {
    None,
    Aes128,
    SampleAES
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct M3u8EncryptKey {
    pub ty: KeyType,
    iv: Option<String>,
    content: Vec<u8>
}

impl Default for M3u8EncryptKey {
    fn default() -> Self {
        M3u8EncryptKey {
            ty: KeyType::None,
            iv: None,
            content: vec![]
        }
    }
}

impl M3u8EncryptKey {
    pub async fn from_key(base_url: Url, k: &m3u8Key) -> anyhow::Result<Self> {
        let key_url = base_url.join(k.uri.as_ref().unwrap()).unwrap();
        Ok(match k.method {
            KeyMethod::None => {
                M3u8EncryptKey {
                    ty: KeyType::None,
                    iv: k.iv.clone(),
                    content: vec![]
                }
            },
            KeyMethod::AES128 => {
                M3u8EncryptKey {
                    ty: KeyType::Aes128,
                    iv: k.iv.clone(),

                    content: download(&key_url).await?
                }
            }
            KeyMethod::SampleAES => {
                M3u8EncryptKey {
                    ty: KeyType::SampleAES,
                    iv: k.iv.clone(),
                    content: download(&key_url).await?
                }
            }
            _ => panic!("{}", format!("Unsupported key method: {}", &k.method))
        })
    }

    pub fn decode(&self, data: &[u8]) -> Vec<u8> {
        let decryptor = crypto::aessafe::AesSafe128Decryptor::new(&self.content);
        let mut reader = AesReader::new(Cursor::new(data), decryptor).unwrap();
        let mut v = Vec::new();
        reader.read_to_end(&mut v).unwrap();
        v
    }
}

#[tokio::main]
pub async fn download_file(id: i32) {
    let mut binging = DOWNLOAD_INFO_CACHE.lock().unwrap();
    let download_info = binging.get(id).unwrap();
    let content = download(&Url::parse(&download_info.url).unwrap()).await.unwrap();

    let url = Url::parse(&download_info.url).unwrap();
    
    match m3u8_rs::parse_playlist_res(&content).expect("Invalid .m3u8 format") {
        Playlist::MasterPlaylist(master) => {
            for (i, vs) in master.variants.iter().enumerate() {
                println!("{index}: \n {stream:#?}", index = format!("#{}", i).blue(), stream = vs)
            }

            let stream = master.variants.get(0).expect(&"Please select a valid number!".red());
            let m3u8_url = url.join(&stream.uri).unwrap();

            let play_list = m3u8_rs::parse_playlist_res(&download(&m3u8_url).await.unwrap()).expect("play list not found");
            match play_list {
                Playlist::MediaPlaylist(media_list) => parse_media_list(m3u8_url, media_list, download_info).await.unwrap(),
                _ => panic!("media play list not found")
            }
        }
        Playlist::MediaPlaylist(list) => {
            parse_media_list(url, list, download_info).await.unwrap();
        }
    }
}

pub async fn parse_media_list(url: Url, play_list: MediaPlaylist, download_info: &mut DownloadInfo) -> anyhow::Result<()> {
    let mut m3u8_encrypt_key = Arc::new(M3u8EncryptKey::default());
    let count = play_list.segments.len();
    download_info.count = count as i32;

    let mut path = utils::app_install_root();
    path.pop();
    let movie_name = download_info.movieName.clone();
    let movie_path = path.join("download").join(&download_info.movieName);
    println!("Cache in {}", &movie_path.to_str().unwrap());

    utils::mkdir(&movie_path);
    let semaphore = Arc::new(Semaphore::new(10));

    let index_path;
    let sub_title_name = &download_info.subTitleName;
    let ts_path;
    if sub_title_name != "" {
        index_path = movie_path.join(&sub_title_name).join(sub_title_name.to_owned() + ".txt");
        ts_path = movie_path.join(&sub_title_name).join("ts");
    } else {
        index_path = movie_path.join(movie_name + ".txt");
        ts_path = movie_path.join("ts");
    }
    
    let mut queue:CustomQueue<DownloadInfoDetail>  = CustomQueue::new();

    for (i, segment) in play_list.segments.iter().enumerate() {

        if let Some(k) = &segment.key {
            let base_key_url = url.clone();
            m3u8_encrypt_key = Arc::new(M3u8EncryptKey::from_key(base_key_url, k).await?)
        } else {
            let index = index_path.clone();
            let ts_path_1 = ts_path.clone();
            if !utils::exists(&index) {
                File::create(&index).await.unwrap();
                let mut index_file = OpenOptions::new().append(true).open(index).unwrap();
                let s = format!("{} {} {}", "file", ts_path_1.join(&segment.uri).into_os_string().into_string().unwrap(), "\n");
                index_file.write(s.as_bytes()).expect("写入文件异常"); 
            }
        }

        // 判断文件碎片是否已下载
        let file_name = movie_path.join(Path::new(&segment.uri).file_name().unwrap());
        if file_name.is_file() {
            if File::open(&file_name).await?.metadata().await?.len() != 0 {
                println!("Pass {}", &file_name.file_name().unwrap().to_str().unwrap());
                continue
            }
        }

        let base_download_url = url.clone();
        let url = base_download_url.join(&segment.uri)?;
        let m3u8_encrypt_key = m3u8_encrypt_key.clone();
        let semaphore = semaphore.clone();
        queue.enqueue(DownloadInfoDetail{
            id: i,
            url,
            m3u8_encrypt_key,
            semaphore,
            file_name,
         });
            
    }
    queue_down(queue).await?;
    merger(path, index_path).await?;
    Ok(())
}

pub async fn queue_down(mut queue: CustomQueue<DownloadInfoDetail>) -> anyhow::Result<()> {
    let tasks = FuturesUnordered::new();
    while queue.size() > 0 {
        let did = queue.dequeue().unwrap();
        let download_count = Arc::new(Mutex::new(0));
        tasks.push(tokio::task::spawn(async move {
            async fn func(detail: DownloadInfoDetail, download_count: Arc<Mutex<i32>>) -> anyhow::Result<()> {
                #[allow(unused_variables)]
                let p = detail.semaphore.acquire().await;

                let resp;
                let mut resp_data;
                loop {
                    resp_data = reqwest::get(detail.url.as_str()).await;
                    match resp_data {
                        Ok(rp) => {
                            if rp.status() != StatusCode::OK {
                                thread::sleep(Duration::from_secs(1));
                            } else {
                                resp = rp;
                                break;
                            }
                        },
                        Err(_e) => {thread::sleep(Duration::from_secs(1));},
                    }
                }

                let mut data = resp.bytes().await?.to_vec();

                if !matches!(&detail.m3u8_encrypt_key.ty, KeyType::None) {
                    data = detail.m3u8_encrypt_key.decode(&mut data);
                }

                let mut file = File::create(&detail.file_name).await?;
                file.write_all(&data).await?;
                let mut download_count = download_count.lock().unwrap();
                *download_count = *download_count + 1;

                Ok(())
            }
            func(did, download_count).await.unwrap();
        }));
    }
    
    futures::future::join_all(tasks).await;

    Ok(())
}

pub async fn merger(path: PathBuf, index_path: PathBuf) -> anyhow::Result<()> {
    let index_str = index_path.into_os_string().into_string().unwrap();
    let mv_str = index_str.replace("txt", "mp4");
    File::create(Path::new(&mv_str)).await.unwrap();
    let output = Command::new(path.join("expand").join("ffmpeg.exe"))
        .arg(format!(" -f concat -i {} -bsf:a aac_adtstoasc -c copy {}", index_str, mv_str))
        .output()
        .expect("视频转码异常!");

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        println!("视频转码成功:{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        println!("视频转码失败:{}", s);
    }

    Ok(())
}

pub async fn download(url: &Url) -> anyhow::Result<Vec<u8>> {
    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("upgrade-insecure-requests"), HeaderValue::from_static("1"));
    let mut base_url = url.clone();
    base_url.set_path("");
    headers.insert(HeaderName::from_static("referer"), HeaderValue::from_str(base_url.as_str()).unwrap());
    
    let client = reqwest::Client::new();
    let resp = client.get(url.as_str()).headers(headers).send().await?;
    if resp.status() != StatusCode::OK {
        panic!("{}", format!("{} download failed. http code: {}", url.as_str(), resp.status()))
    }
    Ok(resp.bytes().await?.to_vec())
}