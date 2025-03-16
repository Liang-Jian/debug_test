use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::json;
use chrono::Local;
use rand::Rng;
use std::error::Error;

const REQ_URL: &str = "127.0.0.1:9000";

// 构造默认请求头
fn default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers
}

/// update: 使用模板下发多页图片
fn update(esl_list: &[String]) -> Result<(), Box<dyn Error>> {
    let mut params = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in esl_list {
        let entry = json!({
            "sid": "19940510",
            "priority": 1,
            "price": rng.gen_range(1..=100).to_string(),
            "template": "new",
            "back_url": "https://172.17.120.30:8087/shopweb-webapp/ogi/ew/httpHandler",
        });
        params.push(entry);
    }
    let data = json!({ "data": params });
    let client = Client::new();
    let url = format!("http://{}/api3/default/esls", REQ_URL);
    let res = client.put(&url)
        .headers(default_headers())
        .json(&data)
        .send()?;
    println!("{}", res.text()?);
    Ok(())
}

/// put_api: 发送 PUT 请求，根据 esl_type 类型不同构造请求
fn put_api(esl_type: &str, store_name: &str) -> Result<(), Box<dyn Error>> {
    // 内部函数：对单个 esl_type 进行操作
    let op = |esl_type_str: &str| -> Result<(), Box<dyn Error>> {
        let data = json!({
            "aps": [
                {
                    "mac": "98:6D:35:79:C5:87",
                    "allow_bind_v1esl": false,
                    "roaming_netlink": false,
                    "mobile_wor": 4
                }
            ]
        });
        let client = Client::new();
        // 构造 URL: /api3/{store_name}/netlink/{esl_type}/bind?force=false
        let url = format!("http://{}/api3/{}/netlink/{}/bind?force=false", REQ_URL, store_name, esl_type_str);
        let res = client.put(&url)
            .headers(default_headers())
            .json(&data)
            .send()?;
        println!("{}", res.text()?);
        println!("finish time in {}", Local::now());
        Ok(())
    };

    op(esl_type)?;
    Ok(())
}

/// get_api: 发送 GET 请求
fn get_api(store_name: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = format!("http://{}/api3/{}/whitelist", REQ_URL, store_name);
    let res = client.get(&url)
        .headers(default_headers())
        .send()?;
    println!("{}", res.text()?);
    println!("finish time in {}", Local::now());
    Ok(())
}

/// delete_api: 发送 DELETE 请求
fn delete_api(store_name: &str) -> Result<(), Box<dyn Error>> {
    let data = json!(["51-59-44-19"]);
    let client = Client::new();
    let url = format!("http://{}/api3/default/esls", REQ_URL);
    let res = client.delete(&url)
        .headers(default_headers())
        .json(&data)
        .send()?;
    println!("{}", res.text()?);
    println!("finish time in {}", Local::now());
    Ok(())
}

/// post_api: 发送 POST 请求
fn post_api() -> Result<(), Box<dyn Error>> {
    let data = json!(["51-59-44-19"]);
    let client = Client::new();
    let url = format!("http://{}/api3/lhy.tt/esls", REQ_URL);
    let res = client.post(&url)
        .headers(default_headers())
        .json(&data)
        .send()?;
    println!("{}", res.text()?);
    println!("finish time in {}", Local::now());
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // 示例调用：设置 esll 为 "30-53-E7-2E" 并调用 put_api
    let esll = "30-53-E7-2E";
    put_api(esll, "default")?;
    // 若需要，可以调用以下函数：
    // update(&vec!["esl1".to_string(), "esl2".to_string()])?;
    // get_api("lhy.tt")?;
    // post_api()?;
    // delete_api("lhy.tt")?;
    Ok(())
}
