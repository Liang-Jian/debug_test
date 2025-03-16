

use reqwest::Client;
use serde_json::json;
use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::Rng;
use tokio::task;

async fn get_sku(fp: &str) -> Vec<String> {
    let file = File::open(fp).expect("Failed to open SKU file");
    let reader = BufReader::new(file);

    let sku_list: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.replace("eslid=", "").trim().to_string())
        .collect();

    sku_list
}

async fn make_sku(client: &Client, store: i32, skuid: &str) {
    let url = "http://127.0.0.1:9091/shopweb/integration";

    let batch_no = format!("make_sku_{}", rand::thread_rng().gen_range(1..1000));

    let payload = json!({
        "customerStoreCode": "SM",
        "storeCode": store.to_string(),
        "batchNo": batch_no,
        "items": [{
            "customerCode": "SM",
            "storeCode": store.to_string(),
            "sku": skuid,
            "itemName": "Avocado Hass",
            "brand": "$3 per ea",
            "unit": "ea",
            "placeOfOrigin": "Australian Grown",
            "level1CategoryName": "WOW",
            "level5CategoryCode": 8,
            "price4Description": "3",
            "promoFlag": 0,
            "supplierCode": "Y",
            "supplierName": "100",
            "manufacturer": "9600000120080586",
            "rsrvTxt1": "N620",
            "rsrvTxt3": "$3 per ea",
            "rsrvDec5": 3.0000,
            "rsrvInt4": 0,
            "rsrvInt5": 1
        }]
    });

    match client.post(url)
        .json(&payload)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                println!("Successfully created SKU: {}", skuid);
            } else {
                println!("Failed to create SKU: {} | Status: {:?}", skuid, response.status());
            }
        }
        Err(err) => println!("Request failed for SKU {}: {:?}", skuid, err),
    }
}

async fn ps_make_sku_self(store: i32) {
    let client = Client::new();
    let sku_list = get_sku("sku.txt").await;

    let mut tasks = vec![];
    for skuid in sku_list {
        let client = client.clone();
        let task = task::spawn(async move {
            make_sku(&client, store, &skuid).await;
        });
        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }
}

async fn run() {
    ps_make_sku_self(1611).await;
}
