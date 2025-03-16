
// 模拟ap登录
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use crc::{Crc, CRC_16_CCITT_FALSE};
use byteorder::{ByteOrder, LittleEndian};
use hex;

const SERVER_IP: &str = "172.17.120.25";  // 服务器 IP
const SERVER_PORT: u16 = 37021;           // 端口号

/// 计算 CRC16
fn calculate_crc16(data: &[u8]) -> u16 {
    let crc16 = Crc::<u16>::new(&CRC_16_CCITT_FALSE);
    crc16.checksum(data)
}

/// 连接服务器
fn connect_to_server() -> TcpStream {
    let address = format!("{}:{}", SERVER_IP, SERVER_PORT);
    loop {
        match TcpStream::connect(&address) {
            Ok(stream) => {
                println!("成功连接到服务器 {}", address);
                return stream;
            }
            Err(e) => {
                println!("连接失败，重试中: {:?}", e);
                thread::sleep(Duration::from_secs(3));
            }
        }
    }
}

/// 发送 AP 登录消息
fn send_ap_login(stream: &mut TcpStream) {
    let msg_hex = "d1070000f405000030303a30383a30363a30393a36373a38380000000000000000";
    let msg = hex::decode(msg_hex).unwrap();
    stream.write_all(&msg).unwrap();
    println!("发送 AP 登录消息: {:?}", msg);
}

/// 发送 AP 心跳消息
fn send_ap_heartbeat(stream: &mut TcpStream) {
    let msg_hex = "D30700000B0000000201040000000025000000";
    let msg = hex::decode(msg_hex).unwrap();
    loop {
        stream.write_all(&msg).unwrap();
        println!("发送 AP 心跳消息: {:?}", msg);
        thread::sleep(Duration::from_secs(30));
    }
}

/// 发送 ESL 任务心跳
fn send_esl_heartbeat(stream: &mut TcpStream) {
    let msg_hex = "41080000130000000102000000000000FC01000002000176000000";
    let msg = hex::decode(msg_hex).unwrap();
    loop {
        stream.write_all(&msg).unwrap();
        println!("发送 ESL 任务心跳: {:?}", msg);
        thread::sleep(Duration::from_secs(10));
    }
}

/// 处理服务器返回的数据
fn receive_data(stream: &mut TcpStream) {
    let mut buffer = [0; 1024]; // 缓冲区
    loop {
        match stream.read(&mut buffer) {
            Ok(size) if size > 0 => {
                println!("接收到数据: {:?}", &buffer[..size]);
                // 解析数据，根据不同的 ID 处理不同的消息
                let msg_id = LittleEndian::read_u32(&buffer[0..4]);
                println!("消息 ID: {}", msg_id);
                
                // 处理不同的消息类型
                match msg_id {
                    2006 => println!("接收到 ESL 任务心跳配置"),
                    2002 => println!("AP 登录成功"),
                    2100 => println!("AP 进行干扰扫描"),
                    _ => println!("未知消息"),
                }
            }
            Ok(_) => {}
            Err(e) => {
                println!("读取数据错误: {:?}", e);
                break;
            }
        }
    }
}

/// 主函数
fn main() {
    let mut stream = connect_to_server();
    
    // 发送 AP 登录
    send_ap_login(&mut stream);

    // 启动 AP 心跳线程
    let mut stream1 = stream.try_clone().unwrap();
    thread::spawn(move || send_ap_heartbeat(&mut stream1));

    // 启动 ESL 任务心跳线程
    let mut stream2 = stream.try_clone().unwrap();
    thread::spawn(move || send_esl_heartbeat(&mut stream2));

    // 监听接收服务器数据
    receive_data(&mut stream);
}
