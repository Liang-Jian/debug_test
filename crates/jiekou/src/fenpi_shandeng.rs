
use reqwest::blocking::Client;
use serde_json::json;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;
use rand::seq::SliceRandom;

/// 配置常量
const CUSTOMER_STORE_CODE: &str = "SM";
const STORE_CODE: &str = "3389";
const SERVER_URL: &str = "http://172.200.9.13/prismart/integration";
const BATCH_SIZE: usize = 7;

/// 读取 ESL 文件
fn read_esl_file(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    reader.lines().filter_map(Result::ok).collect()
}

/// 讲 ESL 分批，每次 `n` 个
fn split_list(collection: Vec<String>, batch_size: usize) -> Vec<Vec<String>> {
    collection.chunks(batch_size).map(|c| c.to_vec()).collect()
}

/// 发送 ESL 变价请求
fn send_shandeng_request(client: &Client, batch: &[String], batch_no: usize) {
    let led_colors = ["blue", "red", "violet", "green", "indigo", "yellow", "white"];
    
    let items: Vec<_> = batch.iter().map(|esl| {
        json!({
            "eslId": esl,
            "IIS_COMMAND": "CUTPAGE_FLASHLIGHTS",
            "IIS_PARAM": {
                "led_count": "5",
                "led_color": led_colors.choose(&mut rand::thread_rng()).unwrap()
            },
            "customerStoreCode": CUSTOMER_STORE_CODE,
            "storeCode": STORE_CODE
        })
    }).collect();

    let payload = json!({
        "customerStoreCode": CUSTOMER_STORE_CODE,
        "storeCode": STORE_CODE,
        "batchNo": format!("{}_shandeng", batch_no),
        "items": items
    });

    let res = client.post(SERVER_URL)
        .json(&payload)
        .send()
        .expect("Failed to send request");

    println!("Sent batch {} at {:?}, waiting 180s...", batch_no, chrono::Utc::now());
    println!("Response: {:?}", res.text().unwrap());
}

/// 主执行函数
fn run() {
    let esl_list = read_esl_file("esl12.txt");
    let batches = split_list(esl_list, BATCH_SIZE);
    let client = Client::new();

    for (batch_no, batch) in batches.iter().enumerate() {
        send_shandeng_request(&client, batch, batch_no + 1);
        thread::sleep(Duration::from_secs(180)); // 等待 180 秒
    }
}
