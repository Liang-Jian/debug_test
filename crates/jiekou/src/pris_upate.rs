

use reqwest::{Client, Error};
use serde_json::json;
use std::{fs, thread, time::Duration};
use tokio;

const REQ_URL: &str = "http://10.10.83.77:9010";
const HEADERS: [(&str, &str); 1] = [("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.212 Safari/537.36")];

#[tokio::main]
async fn main() {
    let mut handles = vec![];
    for store_id in 2..100 {
        let handle = thread::spawn(move || {
            tokio::runtime::Runtime::new().unwrap().block_on(one_store_update(store_id));
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

fn split_list<T: Clone>(list: &[T], chunk_size: usize) -> Vec<Vec<T>> {
    list.chunks(chunk_size).map(|chunk| chunk.to_vec()).collect()
}

async fn one_store_update(store_id: u32) {
    let filename = format!("./template_wumart{}", store_id);
    let esl_list = get_esl_list(&filename);
    let client = Client::new();

    for chunk in split_list(&esl_list, 500) {
        let params: Vec<_> = chunk.iter().map(|e| {
            json!({
                "sid": "3984799300029881",
                "sku": "123456789",
                "esl_id": e,
                "priority": 10,
                "back_url": "http://127.0.0.1:8080",
                "template": "1pageID",
                "itemName": "Cola",
                "price1": 0.99,
                "unit": "1.txt Dose",
                "description": "UEFA CL special",
                "placeOfOrigin": "USA",
                "price5": 1.09,
                "rsrvTxt2": "3",
                "rsrvTxt3": "1€/L",
                "rsrvTxt4": "26",
                "rsrvTxt5": "3",
                "rsrvTxt6": "2",
                "rsrvTxt7": "1,2,3",
                "rsrvTxt10": "1,2,3",
                "qrCode": "1,2,3",
                "rsrvTxt8": "0",
                "rsrvTxt9": "0",
                "rsrvInt2": 3,
                "rsrvTxt1": "000002",
                "rsrvInt1": 500,
                "rsrvDec2": 101,
                "specification": "300",
                "level1CategoryName": "1000",
                "level1CategoryCode": "2",
                "level2CategoryName": "900",
                "level3CategoryName": "880",
                "level4CategoryName": "01.31.2022",
                "level5CategoryName": "L",
                "supervisionHotline": "g",
                "pricingStaff": "10g",
                "price1Description": "15.03.22",
                "supervisedBy": "300g",
                "price2Description": "01.01.2021",
                "price3Description": "15.01.2021",
                "price4Description": "01.02.2022",
                "price5Description": "01.04.2022",
                "price6Description": "1€",
                "force_update": "true"
            })
        }).collect();
        
        let body = json!({"data": params});

        match client.put(&format!("{}/api3/wumart.{}/esls/", REQ_URL, store_id))
            .json(&body)
            .headers(reqwest::header::HeaderMap::from_iter(HEADERS.iter().map(|(k, v)| (k.parse().unwrap(), v.parse().unwrap()))))
            .send()
            .await
        {
            Ok(response) => println!("Store {} update success: {:?}", store_id, response.text().await.unwrap()),
            Err(e) => println!("Store {} error: {:?}", store_id, e),
        }
        
        thread::sleep(Duration::from_millis(1500));
    }
    println!("Store {} update finished, please check shopWeb service", store_id);
}
