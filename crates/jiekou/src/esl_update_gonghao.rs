
use reqwest::{Client, Error};
use serde_json::json;
use std::{fs, thread, time::Duration};
use tokio;
use regex::Regex;
use std::collections::HashMap;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

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

fn analyze_log(log_path: &str) -> HashMap<String, usize> {
    let file = File::open(log_path).expect("Cannot open file");
    let reader = BufReader::new(file);
    let mut log_counts = HashMap::new();
    let re = Regex::new(r"(?i)ack_value|error|failed|success").unwrap();

    for line in reader.lines() {
        let line = line.unwrap();
        for cap in re.find_iter(&line) {
            let key = cap.as_str().to_lowercase();
            *log_counts.entry(key).or_insert(0) += 1;
        }
    }
    log_counts
}

fn save_analysis_to_csv(data: &HashMap<String, usize>, output_path: &str) {
    let mut file = File::create(output_path).expect("Cannot create file");
    writeln!(file, "Metric,Count").unwrap();
    for (key, value) in data {
        writeln!(file, "{},{}", key, value).unwrap();
    }
}

fn get_time_seconds(start: DateTime<Utc>, end: DateTime<Utc>) -> i64 {
    (end - start).num_seconds()
}

fn extract_time(log_line: &str) -> Option<DateTime<Utc>> {
    let re = Regex::new(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\.\d+").unwrap();
    if let Some(cap) = re.find(log_line) {
        let naive_dt = NaiveDateTime::parse_from_str(cap.as_str(), "%Y-%m-%d %H:%M:%S%.f").ok()?;
        Some(DateTime::<Utc>::from_utc(naive_dt, Utc))
    } else {
        None
    }
}
