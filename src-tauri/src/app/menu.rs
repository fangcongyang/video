use tauri::{ AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu };
use tauri_plugin_positioner::on_tray_event;
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

// --- SystemTray Menu
pub fn tray_menu() -> SystemTray {
  SystemTray::new().with_menu(
    SystemTrayMenu::new()
      .add_item(CustomMenuItem::new("restart".to_string(), "重启应用"))
      .add_item(CustomMenuItem::new("quit".to_string(), "退出")),
  )
}

// --- SystemTray Event
pub fn tray_handler(handle: &AppHandle, event: SystemTrayEvent) {
  on_tray_event(handle, &event);

  let app = handle.clone();

  match event {
    SystemTrayEvent::LeftClick { 
      position: _,
      size: _,
      ..
    } => {
      if let Some(main_win) = handle.get_window("main") {
        if !main_win.is_visible().unwrap() {
          main_win.show().unwrap();
        }
        if main_win.is_minimized().unwrap() {
          main_win.unminimize().unwrap()
        }
        main_win.set_focus().unwrap();
      }
    }
    SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
      "restart" => {
        tauri::api::process::restart(&handle.env())
      },
      "quit" => {
        app.save_window_state(StateFlags::all()).expect("保存窗口状态失败"); 
        std::process::exit(0);
      }
      _ => (),
    },
    _ => (),
  }
}

pub mod cmd {
  use tauri::{command, AppHandle, Manager};
  use tauri_plugin_window_state::{AppHandleExt, StateFlags};
  use crate::conf::AppConf;

  #[command]
  pub fn exist_app(app: AppHandle) {
    let main = app.get_focused_window().unwrap();
    let app_conf = AppConf::read();
    let save_window_state = app_conf.systemConf.saveWindowState;
    if app_conf.isinit {
      tauri::api::dialog::ask(
        Some(&app.get_focused_window().unwrap()),
        "退出",
        "你确定退出程序吗？按[x]进行退出",
        move |is_ok| {
            app_conf
            .amend(serde_json::json!({ "isinit" : false, "main_close": is_ok }))
            .write();
            if is_ok {
              if save_window_state {
                app.save_window_state(StateFlags::all()).expect("保存窗口状态失败"); 
              }
              std::process::exit(0);
            } else {
              main.minimize().unwrap();
            }
        },
      );
    } else if app_conf.main_close {
      if save_window_state {
        app.save_window_state(StateFlags::all()).expect("保存窗口状态失败"); 
      }
      std::process::exit(0);
    } else {
      main.minimize().unwrap();
    }
  }
}