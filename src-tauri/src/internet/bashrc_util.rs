use std::fs::{File, write};
use std::io::{BufRead, BufReader};
use std::process::Command;

const HTTP_PROXY: &str = "export all_proxy=";

#[allow(unused)]
pub fn set_proxy(enable: u32, proxy_ip: String) {
    // 打开文件
    let file = File::open("~/.bashrc").expect("无法打开文件");
    // 创建缓冲读取器
    let reader = BufReader::new(file);
    // 获取行迭代器
    let lines = reader.lines();
    let mut new_content = "".to_owned();
    for line in lines {
        if let Ok(content) = line {
            let http_proxy = HTTP_PROXY.to_owned() + &proxy_ip + "\n";
            
            if enable == 0 && content.starts_with(HTTP_PROXY) {
                continue;
            }

            // 检查是否存在该配置项
            if content.starts_with(HTTP_PROXY) {
                new_content += &http_proxy;
            } else {
                new_content += &content;
            }
        }
    }
    // 将修改后的内容写回文件
    write("~/.bashrc", new_content).expect("无法写入文件");
    
    Command::new("source ~/.bashrc")
        .output()
        .expect("无法执行 source 命令");
}
