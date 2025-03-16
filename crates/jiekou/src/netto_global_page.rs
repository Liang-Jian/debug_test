use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use rand::Rng;
use serde_json::json;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

fn global_page() -> Result<(), Box<dyn Error>> {
    // 配置参数
    const EW: &str = "127.0.0.1";
    const PT: u16 = 9000;
    const UC: &str = "shi.003";
    const BAK_URL: &str = "http://localhost:9999/shopweb-webapp/ogi/ew/httpHandler";

    // 构造请求头
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    // 创建 HTTP 客户端
    let client = Client::new();

    // 模拟的 ESL 列表
    let all_esl = vec!["33-FD-5E-4E", "33-FD-67-4E"];

    // 随机取页（返回 1 或 2）
    let mut rng = rand::thread_rng();
    let page: u32 = rng.gen_range(1..3);

    // 计算 timestamp：当前时间 + 2 分钟，取整后乘以 1000（单位毫秒）
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64();
    let timestamp = ((now + 2.0 * 60.0).round() * 1000.0) as u64;

    // 构造参数列表
    let mut params = Vec::new();
    for e in all_esl.iter() {
        let fmt = json!({
            "esl_id": e,
            "sid": "19940602",
            "priority": 10,
            "back_url": BAK_URL,
            "set_cmd": {
                "global_cmd": "CMD_PAGE_CHANGE",
                "set_args": [page, 0],
                "timestamp": timestamp
            }
        });
        params.push(fmt);
    }
    let data = json!({ "data": params });

    // 构造 URL: http://EW:PT/api3/UC/esls/control
    let url = format!("http://{}:{}/api3/{}/esls/control", EW, PT, UC);
    println!("请求 URL: {}", url);

    // 发送 PUT 请求
    let res = client
        .put(&url)
        .headers(headers)
        .json(&data)
        .send()?;

    println!("store_id={} update finished, check service pls", UC);
    println!("Response: {}", res.text()?);

    Ok(())
}
