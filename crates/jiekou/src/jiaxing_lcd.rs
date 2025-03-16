

use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, CONTENT_TYPE};
use serde_json::{json, Value};
use md5;
use std::error::Error;

/// **计算 MD5 哈希值**
fn md5_key(input: &str) -> String {
    format!("{:X}", md5::compute(input))
}

/// **发送 HTTP 请求**
fn re_quest(
    url: &str,
    body: Option<&Value>,
    cookie: Option<&str>,
    method: &str,
    custom_header: Option<&HeaderMap>,
) -> Result<(String, String), Box<dyn Error>> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    
    // 设置默认请求头
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/63.0.3239.132 Safari/537.36 QIHU 360SE"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json;charset=UTF-8"));

    // 添加 Cookie
    if let Some(cookie_value) = cookie {
        headers.insert("Cookie", HeaderValue::from_str(cookie_value)?);
    }

    // 添加自定义请求头
    if let Some(custom_headers) = custom_header {
        headers.extend(custom_headers.clone());
    }

    let response: Response = match method {
        "post" | "POST" => {
            let json_body = serde_json::to_string(body.unwrap_or(&json!({})))?;
            client.post(url).headers(headers).body(json_body).send()?
        }
        _ => client.get(url).headers(headers).send()?,
    };

    let cookie_str = response.headers().get("Set-Cookie").map_or("", |v| v.to_str().unwrap()).to_string();
    let text = response.text()?;

    Ok((cookie_str, text))
}

/// **登录 AllStar 获取 Authorization 令牌**
fn allstar_login() -> Result<(HeaderMap, String), Box<dyn Error>> {
    let allstar_url = "http://172.16.120.191:8081";
    let username = "superuser";
    let password = "superuser";
    let pass_md5 = md5_key(&format!("{}{}", password, username));

    let login_url = format!("{}/proxy/allstar/user/login", allstar_url);
    let body = json!({ "username": username, "password": pass_md5 });

    let (_, text) = re_quest(&login_url, Some(&body), None, "post", None)?;
    let json_data: Value = serde_json::from_str(&text)?;

    println!("{:#?}", json_data);

    let access_token = json_data["data"]["access_token"].as_str().unwrap_or("");
    let auth_header = format!("Bearer {}", access_token);

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", HeaderValue::from_str(&auth_header)?);

    let work_bench_url = format!("{}/proxy/allstar/v2/users/work-bench", allstar_url);
    let (_, work_text) = re_quest(&work_bench_url, None, None, "get", Some(&headers))?;
    let user_id = serde_json::from_str::<Value>(&work_text)?["data"]["userId"].as_str().unwrap_or("").to_string();

    Ok((headers, user_id))
}

fn run() {
    match allstar_login() {
        Ok((headers, user_id)) => {
            println!("Authorization Header: {:#?}", headers);
            println!("User ID: {}", user_id);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
