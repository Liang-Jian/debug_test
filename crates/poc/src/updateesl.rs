
use reqwest::Client;
use serde_json::json;
use std::{error::Error, fs};
use chrono::Local;

/// 对指定的 ESL ID 发送升级请求
async fn api3_upgrade(client: &Client, eslid: &str) -> Result<(), Box<dyn Error>> {
    // 构造请求数据（与 Python 中 json.dumps(update_data) 效果相同）
    let update_data = json!({
        "sid": "3984799300029881",
        "back_url": "http://127.0.0.1:8080/backurl",
        "romfile": "school.zip"
    });

    // 构造 URL，例如： http://172.16.127.72:9000/api3/05.05/esls/<eslid>/rom
    let url = format!("http://172.16.127.72:9000/api3/05.05/esls/{}/rom", eslid);
    
    // 发送 PUT 请求，设置 Content-Type 为 application/json 并提交 JSON 数据
    let resp = client
        .put(&url)
        .header("Content-Type", "application/json")
        .json(&update_data)
        .send()
        .await?;
    
    // 打印响应状态码
    println!("Response status: {}", resp.status());
    // 打印对应 eslid 与 romfile
    println!("{}:{}", eslid, update_data["romfile"]);
    Ok(())
}

/// 读取文件 upgrade_id.txt，将其中的每行作为 ESL ID 发送升级请求
async fn txt_id_run(client: &Client) -> Result<(), Box<dyn Error>> {
    // 打印当前时间（格式与 Python 中类似）
    let now = Local::now();
    println!("{}", now.format("%y-%m-%d %H:%M:%S"));
    
    // 读取文件内容
    let contents = fs::read_to_string("upgrade_id.txt")?;
    // 按行遍历，每行处理
    for line in contents.lines() {
        let eslid = line.trim();
        if !eslid.is_empty() {
            api3_upgrade(client, eslid).await?;
        }
    }
    Ok(())
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     // 构造 HTTP 客户端
//     let client = Client::new();
//     // 调用处理函数
//     txt_id_run(&client).await?;
//     Ok(())
// }
