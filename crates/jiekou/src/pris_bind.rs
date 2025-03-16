

use reqwest::{Client, Error};
use serde_json::json;
use std::{fs, thread, time::Duration};
use tokio;

const REQ_URL: &str = "http://10.10.83.77:8080/prismart/integration";
const PEOPLE: usize = 3;

#[tokio::main]
async fn main() {
    let esl_list = get_esl_list("./prsmt.txt");
    let split_lists = split_list(&esl_list, PEOPLE);
    let mut handles = vec![];

    for esls in split_lists {
        let handle = thread::spawn(move || {
            tokio::runtime::Runtime::new().unwrap().block_on(bind(esls));
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

fn get_esl_list(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Failed to read file: {}", filename))
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn split_list<T: Clone>(list: &[T], parts: usize) -> Vec<Vec<T>> {
    let chunk_size = (list.len() as f64 / parts as f64).ceil() as usize;
    list.chunks(chunk_size).map(|chunk| chunk.to_vec()).collect()
}

async fn bind(esls: Vec<String>) {
    let client = Client::new();
    for esl in esls {
        let body = json!({
            "customerStoreCode": "wumart",
            "storeCode": "001",
            "batchNo": "8",
            "callbackUrl": "",
            "batchSize": 1,
            "items": [{
                "sku": "1",
                "IIS_COMMAND": "BIND",
                "eslId": esl,
                "customerStoreCode": "wumart",
                "storeCode": "001"
            }]
        });

        match client.post(REQ_URL).json(&body).send().await {
            Ok(response) => println!("{} = {:?}", esl, response.text().await.unwrap()),
            Err(e) => println!("Error binding {}: {:?}", esl, e),
        }

        thread::sleep(Duration::from_secs(5));
    }
}
