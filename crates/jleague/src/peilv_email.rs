
use chrono::{Local, Timelike, Datelike};
use reqwest::blocking::Client;
use serde_json::{Value, json};
use std::{fs, thread, time::Duration, collections::HashMap};
use std::io::Write;

const API_URL: &str = "http://your-api-endpoint.com"; // 替换为你的 API 地址
const DATA_FILE: &str = "data.json";
const RESULT_FILE: &str = "result.txt";

fn fetch_data() -> HashMap<String, i64> {
    let client = Client::new();
    let mut mac_uptime_map = HashMap::new();

    match client.get(API_URL).send() {
        Ok(response) => {
            if let Ok(text) = response.text() {
                if let Ok(json_value): Result<Value, _> = serde_json::from_str(&text) {
                    if let Some(mac) = json_value["mac"].as_str() {
                        if let Some(uptime_sec) = json_value["msg"]["up_time_sec"].as_i64() {
                            mac_uptime_map.insert(mac.to_string(), uptime_sec);
                        }
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("API 请求失败: {:?}", err);
        }
    }

    mac_uptime_map
}

fn save_data(data: &HashMap<String, i64>) {
    if let Ok(json) = serde_json::to_string_pretty(data) {
        fs::write(DATA_FILE, json).expect("无法保存数据文件");
    }
}

fn load_previous_data() -> HashMap<String, i64> {
    if let Ok(content) = fs::read_to_string(DATA_FILE) {
        if let Ok(data) = serde_json::from_str(&content) {
            return data;
        }
    }
    HashMap::new()
}

fn compare_uptime(prev_data: &HashMap<String, i64>, new_data: &HashMap<String, i64>) {
    let mut result_file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(RESULT_FILE)
        .expect("无法打开 result.txt 文件");

    for (mac, new_uptime) in new_data {
        if let Some(prev_uptime) = prev_data.get(mac) {
            if new_uptime < prev_uptime {
                let log_entry = format!(
                    "[{}] MAC {} 的 uptime_sec 变小 ({} -> {}), 可能发生重启\n",
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    mac, prev_uptime, new_uptime
                );
                result_file.write_all(log_entry.as_bytes()).expect("无法写入日志");
                println!("{}", log_entry);
            }
        }
    }
}

fn run() {
    loop {
        let now = Local::now();
        if now.hour() == 12 {
            println!("开始查询 API 并存储数据...");
            let new_data = fetch_data();
            let prev_data = load_previous_data();
            compare_uptime(&prev_data, &new_data);
            save_data(&new_data);
            println!("数据处理完成，等待 24 小时...");
            thread::sleep(Duration::from_secs(24 * 3600));
        } else {
            println!("当前时间: {}:{}，等待到 12:00 再执行...", now.hour(), now.minute());
            thread::sleep(Duration::from_secs(60));
        }
    }
}
