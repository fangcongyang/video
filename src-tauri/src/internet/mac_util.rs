use std::process::Command;

const DEVICE_NAME: &str = "Wi-Fi";
const DEFAULT_IGNORE_IP: &str = "192.168.0.0/16 10.0.0.0/8 172.16.0.0/12 127.0.0.1 localhost *.local timestamp.apple.com sequoia.apple.com seed-sequoia.siri.apple.com";

#[allow(unused)]
pub fn set_windows_proxy(enable: u32, proxy_ip: String) {
    let mut ip_iter = proxy_ip.split("\\:");
    if enable == 1 {
        let ip = ip_iter.next().unwrap();
        let port = ip_iter.next().unwrap();
        Command::new(format!("networksetup -setwebproxy {} {} {}", DEVICE_NAME, ip, port));
        Command::new(format!("networksetup -setwebproxystate {} on", DEVICE_NAME));
        Command::new(format!("networksetup -setsecurewebproxy {} {} {}", DEVICE_NAME, ip, port));
        Command::new(format!("networksetup -setsecurewebproxystate {} on", DEVICE_NAME));
        Command::new(format!("networksetup -setproxybypassdomains {} {}", DEVICE_NAME, DEFAULT_IGNORE_IP.to_owned()));
    } else {
        Command::new(format!("networksetup -setwebproxystate {} off", DEVICE_NAME));
        Command::new(format!("networksetup -setsecurewebproxystate {} off", DEVICE_NAME));
        Command::new(format!("networksetup -setproxybypassdomains {} \"Empty\"", DEVICE_NAME));
    }
}