use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::json;
use rand::Rng;
use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::{Duration, Instant};

// 全局参数
const EW_IP: &str = "172.17.120.25";
const PORT: u16 = 9060;
const STORE_NAME: &str = "shi.002";
const BAK_URL: &str = "http://10.11.163.211:8080/shopweb-webapp/ogi/ew/httpHandler";

fn default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers
}

/// 根据模板文件下发数据
fn send_template(template_file: &str, template_type: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    // 打开模板文件，逐行读取（每行作为一个 ESL ID）
    let file = fs::File::open(template_file)?;
    let reader = BufReader::new(file);

    // 构造 timestamp: 当前时间 + 2分钟（单位毫秒）
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs_f64();
    let timestamp = ((now + 2.0 * 60.0).round() * 1000.0) as u64;

    // 根据模板类型构造不同的 JSON 数据
    for line in reader.lines() {
        let esl = line?.trim().to_string();
        if esl.is_empty() {
            continue;
        }
        let url = format!("http://{}:{}/api3/{}/esls/{}", EW_IP, PORT, STORE_NAME, esl);
        // 这里示例中根据模板类型选择不同的下发数据
        let data = match template_type {
            "template_250122213" => json!({
                "sid": "3984799300029881",
                "priority": 10,
                "back_url": BAK_URL,
                "screen": {
                    "name": esl,
                    "default_page": "normal",
                    "default_page_id": "2",
                    "pages": [
                        {
                            "id": 0,
                            "name": "normal",
                            "image": "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"
                }
            ]
        };
        let client = Client::new();
        let url = format!("http://{}:{}/api3/{}/esls/{}", EW_IP, PORT, STORE_NAME, f);
        let res = client.put(&url)
            .header(CONTENT_TYPE, "application/json")
            .json(&data)
            .send()?;
        println!("{}", res.text()?);
    }
    fs::File::open("template_6409601020")?; // 仅用于关闭文件
    Ok(())
}

fn template_296128290() -> Result<(), Box<dyn Error>> {
    // 示例：实现其他模板函数时逻辑类似，这里仅复制一份不同模板名称
    // 可以将实际的 JSON 数据内容做参数化，避免代码重复
    let file = fs::File::open("template_296128290")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let f = line?.trim().to_string();
        if f.is_empty() { continue; }
        let client = Client::new();
        let url = format!("http://{}:{}/api3/{}/esls/{}", EW_IP, PORT, STORE_NAME, f);
        let data = json!({
            "sid": "3984799300029881",
            "priority": 10,
            "back_url": BAK_URL,
            "screen": {
                "name": f,
                "default_page": "normal",
                "default_page_id": "2",
                "pages": [
                    {
                        "id": 0,
                        "name": "normal",
                        "image": "..." // 这里填写对应图片的 Base64 字符串
                    }
                ]
            }
        });
        let res = client.put(&url)
            .header(CONTENT_TYPE, "application/json")
            .json(&data)
            .send()?;
        println!("{}", res.text()?);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    use std::io::BufReader;
    use std::io::prelude::*;
    use std::fs::File;
    use std::thread;
    use std::time::Instant;

    // 模板函数列表
    let func_list = vec![
        "template_296128290",
        "template_250122213",
        "template_384640750",
        "template_6409601020",
    ];

    let start_time = Instant::now();
    let mut handles = Vec::new();

    for func_name in func_list {
        let func_name = func_name.to_string();
        let handle = thread::spawn(move || -> Result<(), Box<dyn Error>> {
            match func_name.as_str() {
                "template_250122213" => template_250122213(),
                "template_296128290" => template_296128290(),
                "template_384640750" => template_384640750(),
                "template_6409601020" => template_6409601020(),
                _ => {
                    println!("Unknown function");
                    Ok(())
                }
            }
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap()?;
    }
    println!("Total time: {:.2?}", start_time.elapsed());
    Ok(())
}
