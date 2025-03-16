
use reqwest::blocking::{Client, multipart};
use std::fs::File;
use std::io::Read;
use serde_json::json;

const WX_KEY: &str = "c8fe19c2-87e4-4206-b092-93a0aa445195";
const UPLOAD_URL: &str = "https://qyapi.weixin.qq.com/cgi-bin/webhook/upload_media";
const SEND_URL: &str = "https://qyapi.weixin.qq.com/cgi-bin/webhook/send";
const FILE_PATH: &str = "all.txt";

fn upload_file() -> Option<String> {
    let client = Client::new();
    let file = File::open(FILE_PATH).expect("无法打开文件");
    let part = multipart::Part::reader(file).file_name("all.txt").mime_str("text/plain").unwrap();
    let form = multipart::Form::new().part("file", part);

    let url = format!("{}?key={}&type=file", UPLOAD_URL, WX_KEY);
    let response = client.post(&url)
        .multipart(form)
        .send()
        .expect("文件上传失败");

    if response.status().is_success() {
        let json: serde_json::Value = response.json().expect("解析 JSON 失败");
        if let Some(media_id) = json.get("media_id") {
            return Some(media_id.as_str().unwrap().to_string());
        }
    }

    None
}

fn send_message(media_id: &str) {
    let client = Client::new();
    let url = format!("{}?key={}", SEND_URL, WX_KEY);
    let data = json!({
        "msgtype": "file",
        "file": { "media_id": media_id }
    });

    let response = client.post(&url)
        .json(&data)
        .send()
        .expect("发送文件消息失败");

    println!("消息发送结果: {:?}", response.text().unwrap());
}

// fn main() {
//     if let Some(media_id) = upload_file() {
//         println!("上传成功，media_id: {}", media_id);
//         send_message(&media_id);
//     } else {
//         println!("文件上传失败！");
//     }
// }
