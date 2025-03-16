

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write, Read};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use flate2::read::GzDecoder;
use chrono::{DateTime, Local, Utc};
use regex::Regex;
use hex;

// CRC16 查找表
const CRC16TAB: [u16; 256] = [
    0x0000, 0x1021, 0x2042, 0x3063, 0x4084, 0x50a5, 0x60c6, 0x70e7,
    0x8108, 0x9129, 0xa14a, 0xb16b, 0xc18c, 0xd1ad, 0xe1ce, 0xf1ef,
    0x1231, 0x0210, 0x3273, 0x2252, 0x52b5, 0x4294, 0x72f7, 0x62d6,
    0x9339, 0x8318, 0xb37b, 0xa35a, 0xd3bd, 0xc39c, 0xf3ff, 0xe3de,
];

/// 计算 CRC16 校验码
fn crc16(data: &[u8]) -> u16 {
    let mut crc = 0;
    for &byte in data {
        let index = ((crc >> 8) as u8 ^ byte) as usize;
        crc = ((crc << 8) ^ CRC16TAB[index] as u16) & 0xFFFF;
    }
    crc
}

/// 解析 ESL 日志文件
fn parse_esl_log(file_path: &str) -> Result<HashMap<String, String>, std::io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut result: HashMap<String, String> = HashMap::new();

    let re = Regex::new(r"(?P<time>\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})").unwrap();

    for line in reader.lines() {
        let line = line?;
        if let Some(caps) = re.captures(&line) {
            let time = caps.name("time").unwrap().as_str().to_string();
            result.insert(time, line);
        }
    }
    Ok(result)
}

/// 解析二进制数据
fn parse_binary_data(hex_data: &str) -> Result<(u32, usize), Box<dyn std::error::Error>> {
    let bytes = hex::decode(hex_data)?;
    let pid = u32::from_le_bytes(bytes[0..4].try_into()?);
    let length = usize::from_le_bytes(bytes[4..8].try_into()?);
    Ok((pid, length))
}

/// 读取 gzip 压缩文件
fn read_gzip_file(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(file_path)?;
    let decoder = GzDecoder::new(file);
    let reader = BufReader::new(decoder);
    Ok(reader.lines().filter_map(Result::ok).collect())
}

/// 计算当前时间戳（毫秒）
fn get_current_timestamp() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}

/// 生成 HTML 报告
fn generate_html_report(data: &HashMap<String, String>) -> String {
    let mut html = String::from("<html><head><meta charset='utf-8'><title>ESL Report</title></head><body>");
    html.push_str("<h1>ESL Log Report</h1><table border='1'><tr><th>Time</th><th>Log Entry</th></tr>");
    
    for (time, log) in data {
        html.push_str(&format!("<tr><td>{}</td><td>{}</td></tr>", time, log));
    }

    html.push_str("</table></body></html>");
    html
}

/// 保存 HTML 文件
fn save_html_report(file_path: &str, content: &str) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// 发送 TCP 请求
fn send_tcp_request<A: ToSocketAddrs>(address: A, data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut stream = TcpStream::connect(address)?;
    stream.set_read_timeout(Some(Duration::from_secs(2)))?;
    stream.write_all(data)?;
    
    let mut response = vec![0; 1024];
    let size = stream.read(&mut response)?;
    response.truncate(size);
    
    Ok(response)
}

/// 读取文件目录下的所有文件
fn list_files(dir: &str) -> Result<Vec<String>, std::io::Error> {
    let mut files = vec![];
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            files.push(path.display().to_string());
        }
    }
    Ok(files)
}

fn main() {
    let log_file = "esl_log.txt";

    // 解析 ESL 日志
    match parse_esl_log(log_file) {
        Ok(parsed_data) => {
            let html_report = generate_html_report(&parsed_data);
            match save_html_report("report.html", &html_report) {
                Ok(_) => println!("Report saved successfully."),
                Err(e) => eprintln!("Failed to save report: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to parse log: {}", e),
    }

    // 解析二进制数据
    let test_hex = "0a402005008015b0";
    match parse_binary_data(test_hex) {
        Ok((pid, length)) => println!("Parsed Data - PID: {}, Length: {}", pid, length),
        Err(e) => eprintln!("Failed to parse binary data: {}", e),
    }

    // 计算 CRC16
    let sample_data = b"Hello ESL";
    let crc_value = crc16(sample_data);
    println!("CRC16 Checksum: {:04X}", crc_value);

    // 读取 gzip 日志
    match read_gzip_file("log.gz") {
        Ok(lines) => println!("Read {} lines from gzip file.", lines.len()),
        Err(e) => eprintln!("Failed to read gzip file: {}", e),
    }

    // 获取当前时间戳
    let timestamp = get_current_timestamp();
    println!("Current Timestamp: {}", timestamp);

    // 发送 TCP 请求
    let tcp_data = b"Test TCP Data";
    match send_tcp_request("127.0.0.1:8080", tcp_data) {
        Ok(response) => println!("Received TCP response: {:?}", response),
        Err(e) => eprintln!("Failed to send TCP request: {}", e),
    }

    // 列出目录中的文件
    match list_files(".") {
        Ok(files) => println!("Files in directory: {:?}", files),
        Err(e) => eprintln!("Failed to list files: {}", e),
    }
}
