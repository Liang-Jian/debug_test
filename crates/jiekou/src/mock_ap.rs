

use rand::Rng;
use std::collections::HashMap;
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

// CRC16 计算表
const CRC16_TABLE: [u16; 256] = [
    0x0000, 0x1021, 0x2042, 0x3063, 0x4084, 0x50A5, 0x60C6, 0x70E7,
    0x8108, 0x9129, 0xA14A, 0xB16B, 0xC18C, 0xD1AD, 0xE1CE, 0xF1EF,
    0x1231, 0x0210, 0x3273, 0x2252, 0x52B5, 0x4294, 0x72F7, 0x62D6,
    0x9339, 0x8318, 0xB37B, 0xA35A, 0xD3BD, 0xC39C, 0xF3FF, 0xE3DE,
];

/// 计算 CRC16
fn calculate_crc16(data: &[u8]) -> u16 {
    let mut crc: u16 = 0;
    for &byte in data {
        let index = ((crc >> 8) ^ byte as u16) as usize;
        crc = (crc << 8) ^ CRC16_TABLE[index];
    }
    crc
}

/// 生成随机 ESL ID
fn generate_esl_id(store_id: u32, ap_id: u32, esl_num: u32) -> String {
    format!(
        "{:02X}-0{}-{:02X}{:02X}",
        store_id % 256,
        ap_id,
        (esl_num >> 8) & 0xFF,
        esl_num & 0xFF
    )
}

/// 连接到 TCP 服务器
fn connect_to_server<A: ToSocketAddrs>(addr: A) -> Option<TcpStream> {
    match TcpStream::connect(addr) {
        Ok(stream) => Some(stream),
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            None
        }
    }
}

/// 发送 AP 登录数据
fn send_ap_login(stream: &mut TcpStream, store_id: u32, ap_id: u32) {
    let login_data = format!("AP_LOGIN:{}:{}", store_id, ap_id);
    let _ = stream.write_all(login_data.as_bytes());
}

/// 发送 AP 心跳数据
fn send_ap_heartbeat(stream: &mut TcpStream) {
    let heartbeat_data = "AP_HEARTBEAT";
    let _ = stream.write_all(heartbeat_data.as_bytes());
}

/// 发送 ESL 心跳数据
fn send_esl_heartbeat(stream: &mut TcpStream, store_id: u32, ap_id: u32, esl_num: u32) {
    let esl_id = generate_esl_id(store_id, ap_id, esl_num);
    let heartbeat_data = format!("ESL_HEARTBEAT:{}", esl_id);
    let _ = stream.write_all(heartbeat_data.as_bytes());
}

/// 监听服务器返回的数据
fn listen_server(mut stream: TcpStream, shared_map: Arc<Mutex<HashMap<String, String>>>) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size == 0 {
                    break;
                }
                let response = String::from_utf8_lossy(&buffer[..size]);
                println!("Received: {}", response);

                let mut map = shared_map.lock().unwrap();
                map.insert("last_response".to_string(), response.to_string());
            }
            Err(_) => break,
        }
    }
}

/// 模拟 ESL 设备
fn simulate_esl(store_id: u32, ap_id: u32, esl_num: u32, server_addr: &str) {
    if let Some(mut stream) = connect_to_server(server_addr) {
        send_ap_login(&mut stream, store_id, ap_id);
        thread::sleep(Duration::from_secs(1));

        loop {
            send_esl_heartbeat(&mut stream, store_id, ap_id, esl_num);
            thread::sleep(Duration::from_secs(30));
        }
    }
}

fn main() {
    let server_addr = "127.0.0.1:37021";

    // 共享存储，用于存储服务器响应
    let shared_map = Arc::new(Mutex::new(HashMap::new()));

    // 生成多个 ESL 设备并启动线程
    let mut handles = vec![];

    for store_id in 1..=3 {
        for ap_id in 1..=2 {
            for esl_num in 10000..=10010 {
                let addr = server_addr.to_string();
                let shared_map_clone = Arc::clone(&shared_map);

                let handle = thread::spawn(move || {
                    simulate_esl(store_id, ap_id, esl_num, &addr);
                });

                handles.push(handle);
            }
        }
    }

    // 启动服务器监听线程
    if let Some(stream) = connect_to_server(server_addr) {
        let shared_map_clone = Arc::clone(&shared_map);
        let handle = thread::spawn(move || listen_server(stream, shared_map_clone));
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        let _ = handle.join();
    }
}
