
use reqwest::blocking::Client;
use serde_json::json;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::seq::SliceRandom;

/// 配置参数
const CUSTOMER_STORE_CODE: &str = "SM";
const STORE_CODE: &str = "3389";
const SERVER_URL: &str = "http://172.200.9.13/prismart/integration";

/// 读取文件内容，返回字符串向量
fn read_file_lines(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    reader.lines().filter_map(Result::ok).collect()
}

/// 将 ESL 列表按指定线程数分割
fn split_list(collection: Vec<String>, num_threads: usize) -> Vec<Vec<String>> {
    let chunk_size = (collection.len() as f64 / num_threads as f64).ceil() as usize;
    collection.chunks(chunk_size).map(|c| c.to_vec()).collect()
}

/// 随机绑定 ESL
fn bind_esl_random(client: &Client, esls: &[String], skus: &[String]) {
    let batch_no = "88";
    let mut items = Vec::new();

    for esl in esls {
        let sku = skus.choose(&mut rand::thread_rng()).unwrap().to_string();
        let item = json!({
            "sku": sku,
            "IIS_COMMAND": "BIND",
            "eslId": esl,
            "customerStoreCode": CUSTOMER_STORE_CODE,
            "storeCode": STORE_CODE
        });
        items.push(item);
    }

    let payload = json!({
        "customerStoreCode": CUSTOMER_STORE_CODE,
        "storeCode": STORE_CODE,
        "batchNo": batch_no,
        "items": items
    });

    let res = client.post(SERVER_URL)
        .json(&payload)
        .send()
        .expect("Failed to send request");

    println!("Random ESL Binding Completed: {:?}", res.text().unwrap());
}

/// 顺序绑定 ESL
fn bind_esl_sorted(client: &Client, esls: &[String], skus: &[String]) {
    if esls.len() != skus.len() {
        panic!("ESL and SKU lists must have the same length!");
    }

    let batch_no = "88";
    let mut items = Vec::new();

    for (esl, sku) in esls.iter().zip(skus.iter()) {
        let item = json!({
            "sku": sku,
            "IIS_COMMAND": "BIND",
            "eslId": esl,
            "customerStoreCode": CUSTOMER_STORE_CODE,
            "storeCode": STORE_CODE
        });
        items.push(item);
    }

    let payload = json!({
        "customerStoreCode": CUSTOMER_STORE_CODE,
        "storeCode": STORE_CODE,
        "batchNo": batch_no,
        "items": items
    });

    let res = client.post(SERVER_URL)
        .json(&payload)
        .send()
        .expect("Failed to send request");

    println!("Sorted ESL Binding Completed: {:?}", res.text().unwrap());
}

/// 主执行函数
fn main() {
    let esl_list = read_file_lines("esl12.txt");
    let sku_list = read_file_lines("sku.txt");
    let num_threads = 3; // 线程数

    let esl_chunks = split_list(esl_list.clone(), num_threads);
    let client = Arc::new(Client::new());

    let mut handles = vec![];

    for chunk in esl_chunks {
        let client_clone = Arc::clone(&client);
        let skus = sku_list.clone();

        let handle = thread::spawn(move || {
            bind_esl_sorted(&client_clone, &chunk, &skus);
            thread::sleep(Duration::from_secs(3));
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
