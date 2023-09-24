use std::{
    num::NonZeroUsize,
    sync:: Mutex,
    fs::{File, create_dir_all}
};
use diesel::{SqliteConnection, Connection};
use lazy_static::lazy_static;
use lru::LruCache;

use crate::utils;
use crate::AppConf;
use crate::data;

pub const DBNAME: &str = "video";

// 定义一个缓存结构体，包含一个lru缓存和一个容量
pub struct Cache {
    lru: LruCache<String, rusqlite::Connection>,
}

// 实现缓存结构体的方法
impl Cache {
    // 创建一个新的缓存，指定容量
    fn new(capacity: usize) -> Cache {
        Cache {
            lru: LruCache::new(NonZeroUsize::new(capacity).unwrap()),
        }
    }

    // 向缓存中插入一个数据库连接，如果缓存已满，淘汰最久未使用的对象
    fn put(&mut self, id: String, channel: rusqlite::Connection) {
        self.lru.put(id, channel);
    }

    // 从缓存中获取一个数据库连接，如果存在，返回Some，否则返回None，并更新最近使用时间
    pub fn get(&mut self, id: String) -> Option<&rusqlite::Connection> {
        self.lru.get(&id)
    }
}

lazy_static! {
    pub static ref CACHE: Mutex<Cache> = Mutex::new(Cache::new(10));
}


pub fn init() {
    let video_db = get_db_conn();
    CACHE.lock().unwrap().put(DBNAME.into(), video_db);
    let app_conf = AppConf::read();
    if !app_conf.isInitDatabase {
        init_database();
        data::site::check_init_site();
        app_conf
        .amend(serde_json::json!({ "isInitDatabase" : true }))
        .write();
    }
    
}

pub fn get_db_conn() -> rusqlite::Connection {
    let path = utils::app_root();
    let output = path.join("data").join("video.db");
    let prefix = output.parent().unwrap();
    create_dir_all(prefix).unwrap();
    if !utils::exists(&output) {
        File::create(&output).unwrap();
    }
    rusqlite::Connection::open(output).unwrap()
}

pub fn get_once_db_conn() -> SqliteConnection {
    let path = utils::app_root();
    let output = path.join("data").join("video.db");
    let prefix = output.parent().unwrap();
    create_dir_all(prefix).unwrap();
    if !utils::exists(&output) {
        File::create(&output).unwrap();
    }
    SqliteConnection::establish(&utils::get_path_name(output)).unwrap()
}

fn init_database() {
    let mut binding = CACHE.lock().unwrap();
    let video_db = binding.get(DBNAME.into()).unwrap();
    
    // 网站信息表
    video_db.execute(
        "create table if not exists site (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL , --主键id
            key TEXT(50) NOT NULL  , --编码
            name TEXT(100) NOT NULL  , --名称
            api TEXT(255) NOT NULL  , --网站api地址
            `group` TEXT(4) NOT NULL  , --影视
            is_active TEXT(1) NOT NULL  , --是否激活;1 激活 0不激活
            status TEXT(3) NOT NULL  , --状态;可用 不可用
            position REAL NOT NULL, --排序
            in_reverse_order NOT NULL --网站分页降序; 1降序 0升序 
        )",
        (),
    ).unwrap();

    // 频道分组表
    video_db.execute(
        "create table if not exists channel_group (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL , --主键id
            name TEXT(50) NOT NULL  , --名称
            channel_group TEXT(20) NOT NULL  , --频道分组
            active TEXT(1) NOT NULL  , --是否激活;1 激活 0不激活
            status TEXT(3) NOT NULL  , --状态;可用 不可用
            has_children TEXT(1) NOT NULL,  --是否有子数据
            channels TEXT(1000) NOT NULL --频道列表
        )",
        (),
    ).unwrap();
    
    // 播放记录表
    video_db.execute(
        "create table if not exists history (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL , --主键id
            name TEXT(50) NOT NULL  , --名称
            ids TEXT(50) NOT NULL  , --网站资源唯一id;网站id+视频id
            `index` INTEGER NOT NULL  , --播放集数
            start_position INTEGER   , --开始跳过时间
            end_position INTEGER   , --结束跳过时间
            play_time REAL   , --已播放时长
            site_key TEXT(50) NOT NULL  , --网站key
            online_play TEXT(500)   , --在线播放
            detail TEXT(2000) NOT NULL,  --明细信息
            video_flag TEXT(50) NOT NULL,  --视频标识
            duration REAL  ,  --视频时长
            has_update TEXT(1) NOT NULL,  --是否有更新
            update_time TEXT(20)  --更新时间
        )",
        (),
    ).unwrap();
    
    // 收藏记录表
    video_db.execute(
        "create table if not exists star (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL , --主键id
            name TEXT(50) NOT NULL  , --名称
            ids TEXT(50) NOT NULL  , --网站资源唯一id;网站id+视频id
            site_key TEXT(50) NOT NULL  , --网站key
            movie_type TEXT(10)  , --影片类型
            year TEXT(10)  , --上映时间
            note TEXT(10)  , --影片备注
            douban_rate TEXT(5)  , --影片豆瓣评分
            has_update TEXT(1) NOT NULL,  --是否有更新
            last_update_time TEXT(20) NOT NULL, --上次更新时间
            position REAL NOT NULL, --排序
            pic TEXT(250) , --图片
            area TEXT(50) NOT NULL -- 区域
        )",
        (),
    ).unwrap();
     
    // 搜索记录表
    video_db.execute(
        "create table if not exists search_record (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL , --主键id
            keywords TEXT(50) NOT NULL  --名称
        )",
        (),
    ).unwrap();

     // 网站解析表
     video_db.execute(
        "create table if not exists website_parse (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL , --主键id
            website_key TEXT(50) NOT NULL  , --网站key
            website_parse_url TEXT(300) NOT NULL  --网站解析地址
        )",
        (),
    ).unwrap();

    // 快捷键信息表
    video_db.execute(
        "create table if not exists shortcut (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL , --主键id
            key TEXT(50) NOT NULL  , --键名
            name TEXT(100) NOT NULL  , --快捷键名称
            desc TEXT(255) NOT NULL  --快捷键描述
        )",
        (),
    ).unwrap();

     // 下载信息表
     video_db.execute(
        "create table if not exists download_info (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL , --主键id
            movie_name TEXT(50) NOT NULL  , --视频名称
            url TEXT(500) NOT NULL  , --视频下载地址
            sub_title_name TEXT(50) NOT NULL  , --子标题名称，连续剧集数名称
            status TEXT(20) NOT NULL  , --状态
            download_count INTEGER  , --下载分片总数
            count INTEGER  , --分片总数
            download_status TEXT(20) NOT NULL  --下载状态
        )",
        (),
    ).unwrap();
}

pub mod cmd {
    use tauri::command;

    #[command]
    pub fn init_database() {
        super::init_database()
    }
}

