

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use reqwest::blocking::{Client, Response};
use serde_json::json;

/// **读取 ESL ID 文件**
fn get_esl_ids(filename: &str) -> io::Result<Vec<String>> {
    let mut esl_ids = Vec::new();
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    
    for line in reader.lines() {
        let line = line?;
        if line.contains('-') {
            let esl_id = line.trim().to_string();
            if !esl_ids.contains(&esl_id) {
                esl_ids.push(esl_id);
            }
        }
    }
    
    println!("TXT 里的价签数量: {}", esl_ids.len());
    Ok(esl_ids)
}

/// **发送 API 请求**
fn send_request(ew_ip: &str, port: u16, store_name: &str, esl_id: &str, cmd: &str, args: &[serde_json::Value]) -> reqwest::Result<()> {
    let url = format!("http://{}:{}/api3/{}/esls/{}/control", ew_ip, port, store_name, esl_id);
    let client = Client::new();
    let data = json!({
        "sid": "3984799300029881",
        "priority": 10,
        "back_url": "http://127.0.0.1:8080/backurl",
        "set_cmd": {
            "global_cmd": cmd,
            "set_args": args,
            "timestamp": "0"
        }
    });

    let response: Response = client.put(&url).json(&data).send()?;
    let response_text = response.text()?;
    
    println!("{} -> {}", esl_id, response_text);
    Ok(())
}

/// **批量执行命令**
fn batch_execute(ew_ip: &str, port: u16, store_name: &str, esl_ids: &[String], cmd: &str, args: Vec<serde_json::Value>) {
    for esl_id in esl_ids {
        if let Err(e) = send_request(ew_ip, port, store_name, esl_id, cmd, &args) {
            eprintln!("Error sending request for {}: {}", esl_id, e);
        }
    }
}

fn run() -> io::Result<()> {
    let ew_ip = "127.0.0.1";
    let store_name = "default";
    let port = 9000;
    let filename = "1.txt";  // ESL ID 文件
    
    let esl_ids = get_esl_ids(filename)?;

    // **发送 LED 快闪命令**
    batch_execute(ew_ip, port, store_name, &esl_ids, "CMD_LED_CONFIG", vec![json!(1)]);

    // **发送强制刷新屏幕命令**
    batch_execute(ew_ip, port, store_name, &esl_ids, "CMD_FORCE_REFRESH_SCREEN", vec![json!(130)]);

    Ok(())
}
