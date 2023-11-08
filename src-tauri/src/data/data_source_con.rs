use std::{
    num::NonZeroUsize,
    sync:: Mutex,
    fs::{File, create_dir_all}
};
use diesel::{SqliteConnection, sql_query, RunQueryDsl, r2d2::{ConnectionManager, Pool, PooledConnection}};
use lazy_static::lazy_static;
use lru::LruCache;

use crate::utils;
use crate::AppConf;
use crate::data;

pub const DBNAME: &str = "video";

// 定义一个缓存结构体，包含一个lru缓存和一个容量
pub struct Cache {
    lru: LruCache<String, Pool<ConnectionManager<SqliteConnection>>>,
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
    fn put(&mut self, id: String, channel: Pool<ConnectionManager<SqliteConnection>>) {
        self.lru.put(id, channel);
    }

    // 从缓存中获取一个数据库连接，如果存在，返回Some，否则返回None，并更新最近使用时间
    pub fn get(&mut self, id: String) -> Option<&Pool<ConnectionManager<SqliteConnection>>> {
        self.lru.get(&id)
    }
}

lazy_static! {
    pub static ref CACHE: Mutex<Cache> = Mutex::new(Cache::new(10));
}

pub fn init() {
    
    let video_db = new_connection_pool();
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

pub fn new_connection_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    let path = utils::app_root();
    let output = path.join("data").join("video.db");
    let prefix = output.parent().unwrap();
    create_dir_all(prefix).unwrap();
    if !utils::exists(&output) {
        File::create(&output).unwrap();
    }
    let manager = ConnectionManager::<SqliteConnection>::new(&utils::get_path_name(output));
    let pool = Pool::builder()
        .max_size(15)
        .min_idle(Some(5))
        .build(manager)
        .expect("Failed to create pool.");
    pool
}

pub fn get_once_db_conn() -> PooledConnection<ConnectionManager<SqliteConnection>> {
    let mut binding = CACHE.lock().unwrap();
    let video_db = binding.get(DBNAME.into()).unwrap();
    video_db.get().unwrap()
}

fn init_database() {
    let mut binding = CACHE.lock().unwrap();
    let video_db = binding.get(DBNAME.into()).unwrap();
    let mut conn = video_db.get().unwrap();
    
    // 网站信息表
    sql_query(
        "create table if not exists site (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL , --主键id
            site_key TEXT(50) NOT NULL  , --编码
            site_name TEXT(100) NOT NULL  , --名称
            api TEXT(255) NOT NULL  , --网站api地址
            site_group TEXT(4) NOT NULL  , --影视
            is_active TEXT(1) NOT NULL  , --是否激活;1 激活 0不激活
            status TEXT(3) NOT NULL  , --状态;可用 不可用
            position REAL NOT NULL  , --排序
            is_reverse_order TEXT(1) NOT NULL --网站分页降序; 1降序 0升序 
        )")
        .execute(&mut conn)
        .unwrap();

    // 频道分组表
    sql_query(
        "create table if not exists channel_group (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL , --主键id
            channel_name TEXT(50) NOT NULL  , --名称
            channel_group_name TEXT(20) NOT NULL  , --频道分组
            channel_active TEXT(1) NOT NULL  , --是否激活;1 激活 0不激活
            channel_status TEXT(3) NOT NULL  , --状态;可用 不可用
            position REAL NOT NULL, --排序
            channels TEXT(1000) NOT NULL --频道列表
        )")
        .execute(&mut conn)
        .unwrap();
    
    // 播放记录表
    sql_query(
        "create table if not exists history (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL , --主键id
            history_name TEXT(50) NOT NULL  , --名称
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
        )")
        .execute(&mut conn)
        .unwrap();
    
    // 收藏记录表
    sql_query(
        "create table if not exists star (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL , --主键id
            star_name TEXT(50) NOT NULL  , --名称
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
        )")
        .execute(&mut conn)
        .unwrap();
     
    // 搜索记录表
    sql_query(
        "create table if not exists search_record (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL , --主键id
            keywords TEXT(50) NOT NULL  --名称
        )")
        .execute(&mut conn)
        .unwrap();

     // 网站解析表
    sql_query(
        "create table if not exists website_parse (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL , --主键id
            website_key TEXT(50) NOT NULL  , --网站key
            website_parse_url TEXT(300) NOT NULL  --网站解析地址
        )")
        .execute(&mut conn)
        .unwrap();

    // 快捷键信息表
    sql_query(
        "create table if not exists shortcut (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL , --主键id
            key TEXT(50) NOT NULL  , --键名
            name TEXT(100) NOT NULL  , --快捷键名称
            desc TEXT(255) NOT NULL  --快捷键描述
        )")
        .execute(&mut conn)
        .unwrap();

     // 下载信息表
    sql_query(
        
        "create table if not exists download_info (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL , --主键id
            movie_name TEXT(50) NOT NULL  , --视频名称
            url TEXT(500) NOT NULL  , --视频下载地址
            sub_title_name TEXT(50) NOT NULL  , --子标题名称，连续剧集数名称
            status TEXT(20) NOT NULL  , --状态
            download_count INTEGER  , --下载分片总数
            count INTEGER  , --分片总数
            download_status TEXT(20) NOT NULL  --下载状态
        )")
        .execute(&mut conn)
        .unwrap();
}

pub mod cmd {
    use tauri::command;
    use crate::conf::AppConf;
    #[allow(unused_imports)]
    use crate::internet::{reg_util, bashrc_util, mac_util, proxy_server};

    #[command]
    pub fn init_database() {
        super::init_database()
    }

    #[command]
    pub fn set_system_proxy(enable: u32, proxy_ip: String) {
        let mut app_conf = AppConf::read();
        let proxy_ip1 = proxy_ip.clone();
        app_conf.systemConf.proxyAddr = proxy_ip;
        app_conf.systemConf.proxyEnable = enable == 1;
        if enable == 1 {
            proxy_server::init_proxy_server();
        }
        app_conf.write(); 
        if cfg!(target_os = "windows") {
            reg_util::set_windows_proxy(enable, proxy_ip1);
        } else if cfg!(target_os = "linux") {
            bashrc_util::set_proxy(enable, proxy_ip1);
        } else if cfg!(target_os = "macos") {
            mac_util::set_windows_proxy(enable, proxy_ip1);
        }
    }
}

