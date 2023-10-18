use log::{error, info};
use serde_json::Value;
use std::{collections::BTreeMap, path::PathBuf};

#[cfg(target_os = "macos")]
use tauri::TitleBarStyle;

use crate::utils::{app_root, create_file, exists, self};

// pub const BUY_COFFEE: &str = "https://www.buymeacoffee.com/lencx";

pub const APP_CONF_PATH: &str = "video.conf.json";

macro_rules! pub_struct {
  ($name:ident {$($field:ident: $t:ty,)*}) => {
    #[allow(non_snake_case)]
    #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
    pub struct $name {
      $(pub $field: $t),*
    }
  }
}

pub_struct!(SystemConf {
  theme: String,
  saveWindowState: bool,
  excludeRootClasses: bool,
  rootClassFilter: Vec<String>,
  excludeR18Films: bool,
  r18ClassFilter: Vec<String>,
  moviesViewMode: String,
  starViewMode: String,
  historyViewMode: String,
  shiftTooltipLimitTimes: i32,
  mainWidth: f64,
  mainHeight: f64,
  shortcutModified: bool,
  encryptedPassword: String,
  downloadSavePath: String,
  shortcutEnabled: bool,
});

impl SystemConf {
  pub fn new() -> Self {
    let mut path = utils::app_install_root();
    path.pop();
    let download_save_path = path.into_os_string().into_string().unwrap();
    Self { 
      theme: "theme-light".into(),
      saveWindowState: false,
      excludeRootClasses: false,
      rootClassFilter: Vec::from(["电影".into(), "电影片".into(), "电视剧".into(), "连续剧".into(), "综艺".into(), "动漫".into()]),
      excludeR18Films: false,
      r18ClassFilter: Vec::from(["伦理".into(), "论理".into(), "倫理".into(), "福利".into(), "激情".into(), "理论".into(), "写真".into(), "情色".into(), 
        "美女".into(), "街拍".into(), "赤足".into(), "性感".into(), "里番".into(), "VIP".into()]),
      moviesViewMode: "picture".into(),
      starViewMode: "picture".into(),
      historyViewMode: "picture".into(),
      shiftTooltipLimitTimes: 5,
      mainWidth: 1080.0,
      mainHeight: 720.0,
      shortcutModified: false,
      encryptedPassword: "".into(),
      downloadSavePath: download_save_path,
      shortcutEnabled: true,
    }
  }
}

pub_struct!(MoviesConf {
  searchGroup: String,
  moviesViewMode: String,
});

impl MoviesConf {
  pub fn new() -> Self {
    Self { 
      searchGroup: "全站".into(),
      moviesViewMode: "picture".into(),
    }
  }
}

pub_struct!(PlayerConf {
  volume: f64,
  autoChangeSourceWhenIptvStalling: bool,
  waitingTimeInSec: i32,
  forwardTimeInSec: i32,
});

impl PlayerConf {
  pub fn new() -> Self {
    Self { 
      volume: 0.6, 
      autoChangeSourceWhenIptvStalling: true,
      waitingTimeInSec: 5,
      forwardTimeInSec: 5,
    }
  }
}

pub_struct!(HistoryConf {
  viewMode: String,
});

impl HistoryConf {
  pub fn new() -> Self {
    Self { 
      viewMode: "picture".into(),
    }
  }
}

pub_struct!(AppConf {
  systemConf: SystemConf,
  shortcut: bool,
  moviesConf: MoviesConf,
  global_shortcut: Option<String>,

  // Main Window
  isinit: bool,
  isInitDatabase: bool,
  popup_search: bool,
  main_close: bool,
  main_dashboard: bool,
  ua_window: String,
  titlebar: bool,

  playerConf: PlayerConf,
  historyConf: HistoryConf,
});

impl AppConf {
  pub fn new() -> Self {
    info!("conf_init");
    Self {
      systemConf: SystemConf::new(),
      shortcut: true,
      popup_search: false,
      isinit: true,
      main_close: false,
      main_dashboard: true,
      isInitDatabase: false,
      ua_window: "".into(),
      titlebar: false,
      global_shortcut: None,
      moviesConf: MoviesConf::new(),
      playerConf: PlayerConf::new(),
      historyConf: HistoryConf::new(),
    }
  }

  pub fn file_path() -> PathBuf {
    app_root().join(APP_CONF_PATH)
  }

  pub fn read() -> Self {
    match std::fs::read_to_string(Self::file_path()) {
      Ok(v) => {
        if let Ok(v2) = serde_json::from_str::<AppConf>(&v) {
          v2
        } else {
          error!("conf_read_parse_error");
          Self::default()
        }
      }
      Err(err) => {
        error!("conf_read_error: {}", err);
        Self::default()
      }
    }
  }

  pub fn write(self) -> Self {
    let path = &Self::file_path();
    if !exists(path) {
      create_file(path).unwrap();
      info!("conf_create");
    }
    if let Ok(v) = serde_json::to_string_pretty(&self) {
      std::fs::write(path, v).unwrap_or_else(|err| {
        error!("conf_write: {}", err);
        Self::default().write();
      });
    } else {
      error!("conf_ser");
    }
    self
  }

  pub fn amend(self, json: Value) -> Self {
    let val = serde_json::to_value(&self).unwrap();
    let mut config: BTreeMap<String, Value> = serde_json::from_value(val).unwrap();
    let new_json: BTreeMap<String, Value> = serde_json::from_value(json).unwrap();

    for (k, v) in new_json {
      config.insert(k, v);
    }

    match serde_json::to_string_pretty(&config) {
      Ok(v) => match serde_json::from_str::<AppConf>(&v) {
        Ok(v) => v,
        Err(err) => {
          error!("conf_amend_parse: {}", err);
          self
        }
      },
      Err(err) => {
        error!("conf_amend_str: {}", err);
        self
      }
    }
  }

  #[cfg(target_os = "macos")]
  pub fn titlebar(self) -> TitleBarStyle {
    if self.titlebar {
      TitleBarStyle::Transparent
    } else {
      TitleBarStyle::Overlay
    }
  }

  pub fn get_conf_by_name(self, conf_name: &str) -> String {
    let val = serde_json::to_value(&self).unwrap();
    let config: BTreeMap<String, Value> = serde_json::from_value(val).unwrap();
    if config.contains_key(conf_name) {
      info!("{}", serde_json::to_string(&config.get(conf_name)).unwrap());
      serde_json::to_string(&config.get(conf_name)).unwrap()
    } else {
      error!("配置不存在: {}", conf_name);
      "".into()
    }
  }
}

impl Default for AppConf {
  fn default() -> Self {
    Self::new()
  }
}

pub mod cmd {
  use super::AppConf;
  use tauri::command;

  // #[command]
  // pub fn reset_app_conf() -> AppConf {
  //   AppConf::default().write()
  // }

  #[command]
  pub fn get_conf_by_name(conf_name: &str) -> String {
    AppConf::read().get_conf_by_name(conf_name)
  }

  #[command]
  pub fn config_update(data: serde_json::Value) {
    AppConf::read().amend(serde_json::json!(data)).write();
  }
}
