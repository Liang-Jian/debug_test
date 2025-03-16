

use reqwest::blocking::{Client, Response};
use reqwest::header::COOKIE;
use serde_json::Value;
use chrono::{Local, Duration, NaiveDateTime};
use std::fs::File;
use std::io::{self, Write};
use std::error::Error;

const LOGIN_URL: &str = "http://10.10.83.77:5288/prismart/weblogin";
const DATA_URL: &str = "http://10.10.83.77:5288/prismart/esl/wumart/001?count=20000";
const USERNAME: &str = "superuser";
const PASSWORD: &str = "8b68d74d7f2ae790c501c55417c9b9bb";
const TIMEOUT_HOURS: i64 = 2;

/// **登录获取 sessionid**
fn login(client: &Client) -> Result<String, Box<dyn Error>> {
    let params = serde_json::json!({
        "username": USERNAME,
        "password": PASSWORD
    });

    println!("{} - 开始登录", Local::now().format("%Y-%m-%d %H:%M:%S"));
    
    let resp: Response = client.post(LOGIN_URL)
        .json(&params)
        .send()?;
    
    let json: Value = resp.json()?;
    println!("{} - 登录返回: {:?}", Local::now().format("%Y-%m-%d %H:%M:%S"), json);

    if json["resultCode"] == 1001 {
        if let Some(session) = json["data"]["jsessionid"].as_str() {
            return Ok(session.to_string());
        }
    }

    Err("登录失败，未能获取 sessionid".into())
}

/// **获取 ESL 数据**
fn get_data(client: &Client, sessionid: &str) -> Result<Vec<Value>, Box<dyn Error>> {
    let resp: Response = client.get(DATA_URL)
        .header(COOKIE, format!("JSESSIONID={}", sessionid))
        .send()?;

    let json: Value = resp.json()?;
    
    if json["resultCode"] == 1001 {
        if let Some(result) = json["data"]["resultSet"].as_array() {
            return Ok(result.clone());
        }
    }
    
    Err("获取数据失败".into())
}

/// **处理数据，筛选没有心跳的 ESL**
fn filter_no_heartbeat(data: Vec<Value>) -> Result<(), Box<dyn Error>> {
    let now = Local::now();
    let cutoff_time = now - Duration::hours(TIMEOUT_HOURS);
    let cutoff_timestamp = cutoff_time.timestamp_millis();

    let filename = format!("noheartbeat_{}.csv", now.format("%Y-%m-%d"));
    let mut file = File::create(&filename)?;

    writeln!(file, "eslId,rom,lastHeartbeatTime,generation,firmwareId,firmwareName")?;

    let mut count = 0;
    for item in data {
        if let (Some(last_heartbeat), Some(esl_id)) = (item["lastHeartbeatTime"].as_i64(), item["eslId"].as_str()) {
            if last_heartbeat < cutoff_timestamp {
                let last_time = NaiveDateTime::from_timestamp_millis(last_heartbeat)
                    .unwrap_or_else(|| NaiveDateTime::from_timestamp(0, 0))
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string();
                
                let rom = item["rom"].as_str().unwrap_or("N/A");
                let generation = item["generation"].as_i64().unwrap_or(0);
                let firmware_id = item["firmwareId"].as_i64().unwrap_or(0);
                let firmware_name = item["firmwareName"].as_str().unwrap_or("Unknown");

                writeln!(file, "{},{},{},{},{},{}", esl_id, rom, last_time, generation, firmware_id, firmware_name)?;
                count += 1;
            }
        }
    }

    println!("共找到 {} 个 ESL 设备超过 {} 小时未发送心跳", count, TIMEOUT_HOURS);
    println!("结果已保存至: {}", filename);

    Ok(())
}

/// **主函数**
fn run() -> Result<(), Box<dyn Error>> {
    let client = Client::builder().danger_accept_invalid_certs(true).build()?;

    let sessionid = login(&client)?;
    println!("sessionid: {}", sessionid);

    let data = get_data(&client, &sessionid)?;
    println!("获取到 {} 条数据", data.len());

    filter_no_heartbeat(data)?;

    Ok(())
}
