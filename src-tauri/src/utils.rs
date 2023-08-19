use std::{
    path::{ Path, PathBuf },
    fs::{ self, read_to_string, File},
    env
};

use anyhow::Result;
use pinyin::{to_pinyin_vec, Pinyin};

pub fn app_root() -> PathBuf {
    tauri::api::path::home_dir().unwrap().join(".video")
}

pub fn app_install_root() -> PathBuf {
    env::current_exe().expect("failed to get current exe path")
}

pub fn read_init_data_file(data_name: &str) -> String {
    let mut path = app_install_root();
    path.pop();
    path = path.join("initData").join(data_name);
    let contents = read_to_string(path)
    .expect("Should have been able to read the file");
    contents
}

pub fn exists(path: &Path) -> bool {
    Path::new(path).exists()
}

pub fn create_file(path: &Path) -> Result<File> {
    if let Some(p) = path.parent() {
      fs::create_dir_all(p)?
    }
    File::create(path).map_err(Into::into)
}

pub fn repeat_vars(count: usize) -> String {
    let mut s = "?,".repeat(count);
    s.pop();
    s
}

pub fn mkdir<P: AsRef<Path>>(path: P) {
    std::fs::create_dir_all(&path).expect(&format!("Cannot create directory {}", path.as_ref().to_str().unwrap()))
}

pub fn get_pinyin_first_letter(name: &str) -> String {
    to_pinyin_vec(name, Pinyin::first_letter).join("")
}
