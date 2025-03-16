use reqwest::blocking::Client;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn call_restful_api(esl_id: &str, base_url: &str) -> Result<(), Box<dyn Error>> {
    // 去掉 esl_id 中的 '-' 字符
    let esl_id_no_blash = esl_id.replace("-", "");
    // 构造 resetKey
    let reset_key = format!("{}{}{}", esl_id_no_blash, "5555555555555555", esl_id_no_blash);
    // 构造完整 URL: baseUrl + eslId + "/cipher/" + resetKey
    let url = format!("{}{}{}{}", base_url, esl_id, "/cipher/", reset_key);
    
    // 创建 HTTP 客户端
    let client = Client::new();
    // 发送 DELETE 请求，设置请求头
    let res = client
        .delete(&url)
        .header("content-type", "application/json")
        .send()?;
    
    // 尝试解析返回的 JSON 数据
    let json_value: Value = res.json()?;
    println!("{}", json_value);
    Ok(())
}

// fn main() -> Result<(), Box<dyn Error>> {
//     let base_url = "http://172.17.120.25:9010/api2/esls/";
//     // 打开文件 eslid.txt
//     let file = File::open("eslid.txt")?;
//     let reader = BufReader::new(file);
    
//     for line in reader.lines() {
//         let line = line?;
//         let esl_id = line.trim();
//         if !esl_id.is_empty() {
//             if let Err(e) = call_restful_api(esl_id, base_url) {
//                 println!("Error for {}: {}", esl_id, e);
//             }
//         }
//     }
//     Ok(())
// }
