
use rand::Rng;
use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::{fs, thread, time::Duration};
use chrono::Local;

const CUSTOMER_STORE_CODE: &str = "SM";
const STORE_CODE: &str = "3389";
const URL: &str = "http://172.200.9.13/prismart/integration";
const CHUNK_SIZE: usize = 1000; // 每批 1000 条数据

/// 读取文本文件并返回行列表
fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("无法读取文件: {}", filename))
        .lines()
        .map(|s| s.to_string())
        .collect()
}

/// 按 `CHUNK_SIZE` 分割数据
fn split_into_chunks<T: Clone>(data: &[T], chunk_size: usize) -> Vec<Vec<T>> {
    data.chunks(chunk_size).map(|chunk| chunk.to_vec()).collect()
}

/// 发送变价请求
fn send_price_update(client: &Client, batch_no: i32, sku_list: &[String]) {
    let mut rng = rand::thread_rng();

    let items: Vec<Value> = sku_list
        .iter()
        .map(|sku| {
            let price = (rng.gen_range(20.0..30.0) * 10.0).round() / 10.0;
            json!({
                "sku": sku,
                "IIS_COMMAND": "UPDATE",
                "price4Description": price, "price4": price,
                "price3Description": price, "price3": price,
                "price5Description": price, "price5": price,
                "price6Description": price, "price6": price,
                "price2Description": price, "price2": price,
                "customerStoreCode": CUSTOMER_STORE_CODE,
                "storeCode": STORE_CODE
            })
        })
        .collect();

    let payload = json!({
        "customerStoreCode": CUSTOMER_STORE_CODE,
        "storeCode": STORE_CODE,
        "batchNo": format!("{}_bianjia", batch_no),
        "items": items
    });

    let sleep_time = rng.gen_range(1..10);
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    println!(
        "批次 {} 发送 {} 条数据, 时间: {}，等待 {}s",
        batch_no,
        sku_list.len(),
        now,
        sleep_time
    );

    let res = client.post(URL).json(&payload).send();
    match res {
        Ok(response) => println!("API 响应: {}", response.text().unwrap_or("无响应".to_string())),
        Err(e) => println!("请求失败: {}", e),
    }

    thread::sleep(Duration::from_secs(sleep_time));
}

/// 主要函数
fn patch_bianjia() {
    let sku_list = read_lines("sku.txt");
    let chunks = split_into_chunks(&sku_list, CHUNK_SIZE);
    let client = Client::builder().danger_accept_invalid_certs(true).build().unwrap();

    for (batch_no, chunk) in chunks.iter().enumerate() {
        send_price_update(&client, (batch_no + 1) as i32, chunk);
    }
}

// fn main() {
//     patch_bianjia();
// }
