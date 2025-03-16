use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::json;
use std::error::Error;
use std::fs;
use std::io::Read;

// 全局参数
const EW_IP: &str = "127.0.0.1";
const PORT: u16 = 9000;
const STORE_NAME: &str = "shi.003";
const BACK_URL: &str = "http://10.11.163.211:8080/shopweb-webapp/ogi/ew/httpHandler";

/// 构造默认的请求头
fn default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers
}

/// 读取指定图片文件并返回 Base64 编码字符串
fn pic2base64(fp: &str) -> Result<String, Box<dyn Error>> {
    let mut file = fs::File::open(fp)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(base64::encode(&buffer))
}

/// 下发模板，针对传入的 ESL 列表和图片文件列表
fn one_esl_more_tmp(esl_list: &[&str], pic_list: &[&str], client: &Client) -> Result<(), Box<dyn Error>> {
    // 要求至少提供两张图片
    if pic_list.len() < 2 {
        return Err("需要至少两个图片文件".into());
    }

    // 依次处理每个 ESL
    for &esl in esl_list {
        let esl = esl.trim();
        let url = format!("http://{}:{}/api3/{}/esls/{}", EW_IP, PORT, STORE_NAME, esl);
        
        // 读取图片文件并转换为 Base64
        let image0 = pic2base64(pic_list[0])?;
        let image1 = pic2base64(pic_list[1])?;
        
        // 构造 JSON 数据，下发模板
        let data = json!({
            "sid": "39847993",
            "priority": 10,
            "back_url": BACK_URL,
            "screen": {
                "name": esl,
                "default_page": "normal",
                "default_page_id": "0",
                "pages": [
                    {
                        "id": 0,
                        "name": "normal",
                        "image": image0,
                    },
                    {
                        "id": 1,
                        "name": "promo",
                        "image": image1,
                    }
                ]
            }
        });
        
        let res = client.put(&url)
            .headers(default_headers())
            .json(&data)
            .send()?;
        println!("下发模板到 {} 返回: {}", esl, res.text()?);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // 构造 HTTP 客户端（复用同一客户端）
    let client = Client::new();
    
    // ESL 列表（可以从文件读取，此处直接写入测试数据）
    let esl_list = vec!["57-1C-2D-DD"];
    
    // 图片文件路径列表（请根据实际路径修改）
    let pic_list = vec![
        r"D:\BBIT_ROUND2\eslww_v5-5.0.2rc5\users\_res\templates\SCT\universe7.5.bmp",
        r"D:\BBIT_ROUND2\eslww_v5-5.0.2rc5\users\_res\templates\SCT\universe7.5P広告の品.bmp",
    ];
    
    // 调用下发模板函数
    one_esl_more_tmp(&esl_list, &pic_list, &client)?;
    
    Ok(())
}
