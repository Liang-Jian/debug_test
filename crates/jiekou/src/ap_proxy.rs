// wu dai jizhan daili gongneng 

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

// 定义请求数据结构
#[derive(Serialize, Deserialize)]
struct Data {
    apMac: String,
    sid: String,
    back_url: String,
    r#type: u32,
    data: Option<DataPayload>,
}

#[derive(Serialize, Deserialize)]
struct DataPayload {
    #[serde(rename = "proxy address")]
    proxy_address: Option<String>,
    server_url: Option<String>,
}

async fn ap_proxy(mac: &str, url: &str, port: u16) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let sn = "wumart.001";
    let back_url = "http://192.168.116.180:9000/shopweb/ogi/ew/httpHandler";
    let proxy_address = format!("{}{}",url, port);

    let data = Data {
        apMac: mac.to_string(),
        sid: "33".to_string(),
        back_url: back_url.to_string(),
        r#type: 54, // 1 for reboot, 52 for upgrade, 51 for ap proxy
        data: Some(DataPayload {
            proxy_address: Some(proxy_address),
            server_url: None,
        }),
    };

    let response = client
        .put(format!("http://{}/api3/{}/aps/management", "10.12.63.52:9000", sn))
        .json(&data)
        .send()
        .await?;

    let body = response.text().await?;
    println!("{}", body);

    Ok(())
}

async fn ap_upgrade(mac: &str, url: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let sn = "wumart.001";
    let back_url = "http://192.168.116.180:9000/shopweb/ogi/ew/httpHandler";

    let data = Data {
        apMac: mac.to_string(),
        sid: "33".to_string(),
        back_url: back_url.to_string(),
        r#type: 52, // 52 for upgrade
        data: Some(DataPayload {
            proxy_address: None,
            server_url: Some(url.to_string()),
        }),
    };

    let response = client
        .put(format!("http://{}/api3/{}/aps/management", "10.12.63.52:9000", sn))
        .json(&data)
        .send()
        .await?;

    let body = response.text().await?;
    println!("{}", body);

    Ok(())
}

async fn ap_logupload(mac: &str, url: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let sn = "wumart.001";
    let back_url = "http://192.168.116.180:9000/shopweb/ogi/ew/httpHandler";

    let data = Data {
        apMac: mac.to_string(),
        sid: "33".to_string(),
        back_url: back_url.to_string(),
        r#type: 51, // 51 for log upload
        data: Some(DataPayload {
            proxy_address: None,
            server_url: Some(url.to_string()),
        }),
    };

    let response = client
        .put(format!("http://{}/api3/{}/aps/management", "10.12.63.52:9000", sn))
        .json(&data)
        .send()
        .await?;

    let body = response.text().await?;
    println!("{}", body);

    Ok(())
}

// #[tokio::main]
async fn run() -> Result<(), Box<dyn Error>> {
    let mac = "98:6D:35:7C:35:86";
    let log_url = "https://10.12.63.120:8083";

    // 调用 ap_logupload 函数上传日志
    ap_logupload(mac, log_url).await?;

    Ok(())
}
