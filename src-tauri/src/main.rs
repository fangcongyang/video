// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate diesel;

mod app;
mod data;
mod utils;
mod conf;
mod download;
mod schema;

use conf::AppConf;
use app::{setup, menu};
use data::{ site, data_source_con, channel, history, star, search_record, website_parse, shortcut, download_info::{self} };
use download::file_download;

fn main() {
    AppConf::read().write();
    let context = tauri::generate_context!();

    data_source_con::init();
    shortcut::check_init_shortcut();
    tauri::async_runtime::spawn(file_download::init());
    file_download::init_download_queue();

    let builder = tauri::Builder::default()
    .plugin(tauri_plugin_positioner::init())
    .invoke_handler(tauri::generate_handler![
        site::cmd::select_site,
        site::cmd::get_site_by_key,
        site::cmd::save_site,
        site::cmd::del_site,
        site::cmd::reset_site,
        site::cmd::insert_sites,
        conf::cmd::get_conf_by_name,
        conf::cmd::config_update,
        channel::cmd::select_channel_group,
        channel::cmd::save_channel_group,
        channel::cmd::save_channel_groups,
        channel::cmd::del_channel_group,
        channel::cmd::del_channel_groups,
        history::cmd::select_history,
        history::cmd::save_history,
        history::cmd::insert_historys,
        history::cmd::del_history,
        history::cmd::get_history_by_uq,
        history::cmd::del_all_history,
        history::cmd::del_historys,
        star::cmd::select_star,
        star::cmd::get_star_by_uq,
        star::cmd::save_star,
        star::cmd::insert_stars,
        star::cmd::del_star,
        star::cmd::del_stars,
        search_record::cmd::select_search_record,
        search_record::cmd::save_search_record,
        search_record::cmd::del_all_search_record,
        website_parse::cmd::select_website_parse,
        website_parse::cmd::save_website_parse,
        website_parse::cmd::del_website_parse,
        menu::cmd::exist_app,
        download_info::cmd::select_download_info,
        download_info::cmd::insert_download_infos,
        download_info::cmd::get_download_by_id,
        download_info::cmd::del_download_info,
        shortcut::cmd::select_shortcut,
        shortcut::cmd::save_shortcut,
        shortcut::cmd::reset_shortcut,
        data_source_con::cmd::init_database,
        file_download::cmd::get_download_info_by_queue,
        file_download::cmd::retry_download,
    ])
    .setup(setup::init);
    builder.system_tray(menu::tray_menu())
    .on_system_tray_event(menu::tray_handler)
    .run(context)
    .expect("运行视频软件错误");
}
