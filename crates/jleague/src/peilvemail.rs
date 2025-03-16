

use reqwest::blocking::{Client};
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;
use chrono::{Local};
use csv::Writer;
use sqlx::{MySql, Pool};
use std::collections::HashMap;

const WX_ID: &str = "75f12a00-cf4f-4aee-9296-63caaddcd9ec";
const ID_URL: &str = "https://qyapi.weixin.qq.com/cgi-bin/webhook/upload_media";
const WX_URL: &str = "https://qyapi.weixin.qq.com/cgi-bin/webhook/send";
const API_URL: &str = "https://api.example.com/data";  // API地址
const DB_URL: &str = "mysql://user:password@localhost/db_name"; // 替换为你的数据库连接

fn request_api() -> HashMap<String, i64> {
    let client = Client::new();
    let mut result = HashMap::new();

    match client.get(API_URL).send() {
        Ok(response) => {
            if let Ok(text) = response.text() {
                if let Ok(json_value): Result<Value, _> = serde_json::from_str(&text) {
                    if let Some(mac) = json_value["mac"].as_str() {
                        if let Some(uptime_sec) = json_value["msg"]["up_time_sec"].as_i64() {
                            result.insert(mac.to_string(), uptime_sec);
                        }
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("API 请求失败: {:?}", err);
        }
    }
    result
}

fn save_to_csv(data: &HashMap<String, i64>) {
    let now = Local::now().format("%Y-%m-%d").to_string();
    let filename = format!("data_{}.csv", now);
    let mut wtr = Writer::from_path(&filename).expect("无法创建 CSV 文件");

    wtr.write_record(&["MAC", "Uptime_Sec"]).expect("无法写入 CSV 头");
    for (mac, uptime) in data {
        wtr.write_record(&[mac, &uptime.to_string()]).expect("无法写入 CSV 数据");
    }
    wtr.flush().expect("无法保存 CSV");
}

async fn save_to_mysql(pool: &Pool<MySql>, data: &HashMap<String, i64>) {
    for (mac, uptime) in data {
        sqlx::query("INSERT INTO system_status (mac, uptime_sec) VALUES (?, ?)")
            .bind(mac)
            .bind(*uptime)
            .execute(pool)
            .await
            .expect("数据库插入失败");
    }
}

fn load_previous_data() -> HashMap<String, i64> {
    let now = Local::now().format("%Y-%m-%d").to_string();
    let filename = format!("data_{}.csv", now);
    let mut result = HashMap::new();

    if let Ok(mut rdr) = csv::Reader::from_path(filename) {
        for record in rdr.records() {
            if let Ok(record) = record {
                let mac = record[0].to_string();
                let uptime_sec: i64 = record[1].parse().unwrap_or(0);
                result.insert(mac, uptime_sec);
            }
        }
    }
    result
}

fn compare_uptime(prev_data: &HashMap<String, i64>, new_data: &HashMap<String, i64>) {
    let mut file = File::create("result.txt").expect("无法创建 result.txt 文件");

    for (mac, new_uptime) in new_data {
        if let Some(prev_uptime) = prev_data.get(mac) {
            if new_uptime < prev_uptime {
                let log_entry = format!(
                    "[{}] MAC {} uptime_sec 变小 ({} -> {}), 可能发生重启\n",
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    mac, prev_uptime, new_uptime
                );
                file.write_all(log_entry.as_bytes()).expect("无法写入日志");
                println!("{}", log_entry);
            }
        }
    }
}

fn send_wechat_message(content: &str) {
    let client = Client::new();
    let data = serde_json::json!({
        "msgtype": "text",
        "text": {
            "content": content
        }
    });

    let _ = client.post(WX_URL)
        .json(&data)
        .send();
}

fn run() {
    let pool = Pool::<MySql>::connect(DB_URL).await.expect("无法连接数据库");

    loop {
        let now = Local::now();
        if now.hour() == 12 {
            println!("开始查询 API 并存储数据...");
            let new_data = request_api();
            let prev_data = load_previous_data();
            compare_uptime(&prev_data, &new_data);
            save_to_csv(&new_data);
            save_to_mysql(&pool, &new_data).await;
            send_wechat_message("数据查询完成");
            println!("数据处理完成，等待 24 小时...");
            thread::sleep(Duration::from_secs(24 * 3600));
        } else {
            println!("当前时间: {}:{}，等待到 12:00 再执行...", now.hour(), now.minute());
            thread::sleep(Duration::from_secs(60));
        }
    }
}

#[tokio::main]
async fn main() {
    run().await;
}
