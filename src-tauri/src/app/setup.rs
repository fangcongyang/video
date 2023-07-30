use crate::conf::AppConf;
use log::info;
use tauri::{utils::config::WindowUrl, window::WindowBuilder, App};
use window_shadows::set_shadow;
use tauri_plugin_window_state::{WindowExt, StateFlags};

pub fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
  info!("stepup");
  let app_conf = AppConf::read();
  
  let app_handle: tauri::AppHandle = app.handle();
  tauri::async_runtime::spawn(async move {
    let main_win = WindowBuilder::new(&app_handle, "main", WindowUrl::App("index.html".into()))
      .title("vop")
      .resizable(true)
      .fullscreen(false)
      .inner_size(app_conf.systemConf.mainWidth, app_conf.systemConf.mainHeight)
      .center()
      .decorations(false)
      .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36 Edg/114.0.1823.58".into());

    #[cfg(target_os = "macos")]
    {
      main_win = main_win
        .title_bar_style(app_conf.clone().titlebar())
        .hidden_title(true);
    }

    let main = main_win.build().unwrap();

    if app_conf.systemConf.saveWindowState {
      main.restore_state(StateFlags::all()).expect("还原窗口失败");
    } else {
      #[cfg(any(windows, target_os = "macos"))]
      set_shadow(&main, true).expect("Unsupported platform!");
    }

  });

  // auto_update
  let auto_update = app_conf.get_auto_update();
  if auto_update != "disable" {
    info!("run_check_update");
    // utils::run_check_update(app, auto_update == "silent", None);
  }

  Ok(())
}
