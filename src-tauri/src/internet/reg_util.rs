#[cfg(target_os = "windows")]

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;
use std::ptr::null_mut;
use winapi::shared::minwindef::HKEY;
use winapi::shared::winerror::SEC_E_OK;
use winapi::um::winnt::{KEY_ALL_ACCESS, REG_DWORD, REG_MULTI_SZ};
use winapi::um::winreg::{
    RegOpenKeyExW, RegSetValueExW, HKEY_CURRENT_USER, LSTATUS, RegCloseKey,
};

const INTERNET_SETTINGS: &str = "HKEY_CURRENT_USER\\Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings";
const KEY_PROXY_ENABLE: &str = "ProxyEnable";
const KEY_PROXY_SERVER: &str = "ProxyServer";
// const KEY_PROXY_OVERRIDE: &str = "ProxyOverride";

// 将 &str 类型转为 LPCWSTR 类型
fn str_to_lpcwstr(str: &str) -> Vec<u16> {
    OsStr::new(str).encode_wide().chain(once(0)).collect()
}

// 打开注册表项
fn reg_open(main_hkey: HKEY, sub_key: &str) -> Result<HKEY, String> {
    unsafe {
        let mut hkey: HKEY = null_mut();
        let status = RegOpenKeyExW(main_hkey, str_to_lpcwstr(sub_key).as_ptr(), 0, KEY_ALL_ACCESS, &mut hkey);
        if status == SEC_E_OK {
            return Result::Ok(hkey);
        }
        return Result::Err(format!("status == {}", status));
    }
}

fn reg_save_sz(hkey: &HKEY, key_name: &str, value: &str) -> LSTATUS {
    unsafe {
        let value_data = str_to_lpcwstr(value);
        RegSetValueExW(*hkey, str_to_lpcwstr(key_name).as_ptr(), 0, REG_MULTI_SZ,
            value_data.as_ptr() as *const u8, 
            (value_data.len() - 1) as u32)
    }
}

// fn reg_append_save_sz(hkey: &HKEY, key_name: &str, value: &str) {
//     let mut dtype = 0;
//     let mut dword = 0;
//     let mut data_binary = vec![0; 1024];
//     let value_name = str_to_lpcwstr(key_name).as_ptr();
//     let status = unsafe {
//         RegQueryValueExW(
//             *hkey,
//             value_name,
//             null_mut(),
//             &mut dtype,
//             data_binary.as_mut_ptr(),
//             &mut dword,
//         )
//     };
//     if status != 0 {
//         println!("读取值失败，错误码：{}", status);
//         return;
//     }
//     // 判断值类型和内容
//     if dtype == REG_MULTI_SZ {
//         // 转换为字符串
//         let value_data = String::from_utf8_lossy(&data_binary[..dword as usize / 2]);
//         println!("原始值: {}", value_data);

//         // 追加新地址
//         let new_address = OsStr::new(&(";".to_owned() + value))
//             .encode_wide()
//             .chain(once(0))
//             .collect::<Vec<_>>();
//         data_binary.extend_from_slice(&new_address.iter().map(|&x| x as u8).collect::<Vec<_>>());

//         // 修改值
//         let status = unsafe {
//             RegSetValueExW(
//                 *hkey,
//                 value_name,
//                 0,
//                 REG_MULTI_SZ,
//                 data_binary.as_ptr(),
//                 (data_binary.len() - 1) as u32,
//             )
//         };
//         if status != 0 {
//             println!("修改值失败，错误码：{}", status);
//             return;
//         }

//         println!("修改值成功");
//     } else {
//         println!("值类型不是 REG_MULTI_SZ");
//     }
// }

pub fn set_windows_proxy(enable: u32, proxy_ip: String) {
    let hkey_result = reg_open(HKEY_CURRENT_USER, INTERNET_SETTINGS);
    let hkey = hkey_result.unwrap();
    unsafe { RegSetValueExW(*&hkey, str_to_lpcwstr(KEY_PROXY_ENABLE).as_ptr(), 0, REG_DWORD, &enable as *const _ as *const u8, std::mem::size_of_val(&enable) as u32) };
    let mut proxy_ip_iter = proxy_ip.split("//");
    proxy_ip_iter.next().unwrap();
    reg_save_sz(&hkey, KEY_PROXY_SERVER, proxy_ip_iter.next().unwrap());
    // reg_append_save_sz(&hkey, KEY_PROXY_OVERRIDE, &mut ignore_ip);
    unsafe { RegCloseKey(hkey) };
}