use reqwest::{Client, Proxy};
use tokio::{net::{TcpListener, TcpStream}, io::{BufReader, AsyncBufReadExt, AsyncWriteExt}, runtime::Builder};
use rand::Rng;

use crate::conf::AppConf;

// 定义一个异步函数，用来处理每个连接
async fn handle_connection(mut stream: TcpStream, client: Client) {
    // 创建一个缓冲区，用来存储请求数据
    let mut buffer = Vec::new();
    // 读取请求数据，直到遇到空行
    let mut reader = BufReader::new(&mut stream);
    loop {
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line).await.unwrap();
        if bytes_read == 0 || line == "\r\n" {
            break;
        }
        buffer.extend_from_slice(line.as_bytes());
    }
    // 把请求数据转换为字符串
    let request = String::from_utf8_lossy(&buffer);
    // 解析请求行，获取方法和URL
    let mut parts = request.split_whitespace();
    let method = parts.next().unwrap_or("");
    let url = parts.next().unwrap_or("");
    // 根据方法和URL，构造一个reqwest请求对象
    let req = match method {
        "GET" => client.get(url),
        "POST" => client.post(url),
        "PUT" => client.put(url),
        "DELETE" => client.delete(url),
        "HEAD" => client.head(url),
        _ => return,
    };
    // 发送请求，并等待响应
    let res = req.send().await.unwrap();
    // 获取响应的状态码和内容
    let status = res.status();
    let body = res.bytes().await.unwrap();
    // 构造响应行和响应头
    let response_line = format!("HTTP/1.1 {}\r\n", status);
    let response_headers = format!(
        "Content-Length: {}\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n",
        body.len()
    );
    // 把响应行和响应头写入缓冲区
    buffer.clear();
    buffer.extend_from_slice(response_line.as_bytes());
    buffer.extend_from_slice(response_headers.as_bytes());
    // 把缓冲区的数据写入流
    stream.write_all(&buffer).await.unwrap();
    // 把响应内容写入流
    stream.write_all(&body).await.unwrap();
}

fn local_check_port(port: u16) -> bool {
    let result = std::net::TcpStream::connect(("127.0.0.1", port));
    if result.is_ok() {
        return false;
    } else {
        return true;
    }
}

#[tokio::main]
pub async fn init_proxy_server() {
    let not_use_port;
    loop {
        let mut rng = rand::thread_rng();
        let port = rng.gen_range(65000..65535);
        if local_check_port(port) {
            not_use_port = port;
            break;
        }
    }
    let mut app_conf = AppConf::read();
    app_conf.systemConf.proxyPort = not_use_port;
    app_conf.systemConf.proxyUrl = "127.0.0.1:".to_owned() + &not_use_port.to_string();
    let app_conf1 = app_conf.clone();
    app_conf.write();
    let proxy_url = app_conf1.systemConf.proxyUrl;
    let listener = TcpListener::bind(proxy_url.clone()).await.unwrap();
    let runtime = Builder::new_multi_thread()
        .worker_threads(10)
        .thread_name("vop-proxy-thread")
        .thread_stack_size(1024 * 1024)
        .build()
        .unwrap();
    loop {
        if !AppConf::read().systemConf.proxyEnable {
            break;
        }
        let client = Client::builder().proxy(Proxy::all(app_conf1.systemConf.proxyAddr.clone()).unwrap()).build().unwrap();
        let (stream, _) = listener.accept().await.unwrap();
        runtime.spawn(handle_connection(stream, client.clone()));
    }
}