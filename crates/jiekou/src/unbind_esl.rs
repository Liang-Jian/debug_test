

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::json;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const REQ_URL: &str = "http://127.0.0.1:9000";
const USERCODE: &str = "default";

/// **读取 `esl_id` 文件**
fn get_txt(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let esl_list: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();
    Ok(esl_list)
}

/// **发送 `HTTP DELETE` 请求，解绑 ESL**
fn update_esl(esl_list: Vec<String>) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    
    // 设置 HTTP 头
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    // 构建请求 JSON 数据
    let data = json!({
        "data": esl_list.iter().map(|id| json!({
            "sid": "3984799300029881",
            "priority": 10,
            "esl_id": id,
            "back_url": "https://101.201.81.174:8087/shopweb-webapp/ogi/ew/httpHandler",
            "template": "_UNBIND",
            "set_free": true
        })).collect::<Vec<_>>()
    });

    let url = format!("{}/api3/{}/esls/bind", REQ_URL, USERCODE);

    // 发送 `DELETE` 请求
    let res = client
        .delete(&url)
        .headers(headers)
        .json(&data)
        .send()?;

    println!("Response: {:?}", res.text()?);
    println!("Unbind finish");

    Ok(())
}

fn run() {
    println!("请输入解绑价签的 ID 文件名:");
    let mut eslfile = String::new();
    std::io::stdin().read_line(&mut eslfile).expect("读取输入失败");
    let eslfile = eslfile.trim();

    match get_txt(eslfile) {
        Ok(esl_list) => {
            if let Err(err) = update_esl(esl_list) {
                eprintln!("解绑失败: {}", err);
            }
        }
        Err(err) => eprintln!("读取文件失败: {}", err),
    }
}
