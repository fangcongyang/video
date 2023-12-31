use crate::conf::AppConf;
use log::info;
use tauri::{utils::config::WindowUrl, window::WindowBuilder, App};
use window_shadows::set_shadow;
use tauri_plugin_window_state::{WindowExt, StateFlags};

pub fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
  info!("stepup");
  let app_conf = AppConf::read();
  
  let app_handle: tauri::AppHandle = app.handle();
  let app_conf2 = app_conf.clone();
  tauri::async_runtime::spawn(async move {
    let mut main_win = WindowBuilder::new(&app_handle, "main", WindowUrl::App("index.html".into()))
      .title("vop")
      .resizable(true)
      .fullscreen(false)
      .disable_file_drop_handler()
      .inner_size(app_conf2.systemConf.mainWidth, app_conf2.systemConf.mainHeight)
      .center()
      .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36 Edg/114.0.1823.58".into());

    #[cfg(target_os = "macos")]
    {
      main_win = main_win
        .title_bar_style(app_conf2.clone().titlebar())
        .hidden_title(true);
    }
    #[cfg(not(target_os = "macos"))]
    {
      main_win = main_win.decorations(false);
    }

    let main = main_win.build().unwrap();

    if app_conf2.systemConf.saveWindowState {
      main.restore_state(StateFlags::all()).expect("还原窗口失败");
    } else {
      #[cfg(any(windows, target_os = "macos"))]
      set_shadow(&main, true).expect("Unsupported platform!");
    }

  });

  Ok(())
}
