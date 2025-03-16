use reqwest::Client;
use serde_json::{json, Value};
use std::fs;
use std::error::Error;

/// 从指定文件中读取 ESL 数据，每行形如 "eslid=xxx"，返回 ESL id 的 Vec<String>
fn get_esl(fp: &str) -> Vec<String> {
    let contents = fs::read_to_string(fp).expect("failed to read file");
    contents
        .lines()
        .map(|line| line.replace("eslid=", "").trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// 登录并返回 HTTP 客户端（这里直接使用同一个客户端进行后续请求）
async fn login(client: &Client) -> Result<Value, Box<dyn Error>> {
    let login_url = "http://172.16.127.37:8717/prismart/weblogin/v2";
    let login_data = json!({
        "username": "superuser",
        "password": "769b3fa2c8c1ba02436689febeb97314"
    });

    let resp = client
        .post(login_url)
        .header("Content-Type", "application/json;charset=utf-8")
        .json(&login_data)
        .send()
        .await?;
    let resp_json: Value = resp.json().await?;
    println!("Login response: {}", resp_json);
    Ok(resp_json)
}

/// 绑定 ESL 列表，构造请求数据并 POST 到绑定接口
async fn bind_esl(client: &Client, esl_list: &[String]) -> Result<Value, Box<dyn Error>> {
    // 构造请求数据：数组中每个元素为一个对象
    let data: Vec<Value> = esl_list
        .iter()
        .map(|esl| {
            json!({
                "eslId": esl,
                "goodsSku": "123",
                "position": 0,
                "extra": {}
            })
        })
        .collect();

    let bind_url = "http://172.16.127.37:8717/prismart/esl/wumart/5/binding";
    let resp = client.post(bind_url).json(&data).send().await?;
    let resp_json: Value = resp.json().await?;
    println!("Bind response: {}", resp_json);
    Ok(resp_json)
}

/// 主业务逻辑：读取 ESL、登录并绑定 ESL
async fn del_esl() -> Result<(), Box<dyn Error>> {
    // 读取 ESL 文件（根据需要修改路径）
    let esl_file = "D:\\forever\\src\\esl.txt";
    let esl_list = get_esl(esl_file);
    println!("ESL list: {:?}", esl_list);

    // 构造 HTTP 客户端
    let client = Client::builder().build()?;

    // 登录（这里仅打印登录返回信息）
    let _login_resp = login(&client).await?;

    // 绑定 ESL
    let _bind_resp = bind_esl(&client, &esl_list).await?;
    println!("bind esl finished !");
    Ok(())
}

// #[tokio::main]
// async fn main() {
//     if let Err(e) = del_esl().await {
//         eprintln!("Error: {}", e);
//     }
// }
