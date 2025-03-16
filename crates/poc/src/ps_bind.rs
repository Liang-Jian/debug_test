use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::error::Error;
use std::fs;
use rand::Rng;

/// 从指定文件中读取每一行，去掉前后空白，返回 Vec<String>
fn get_esl(fp: &str) -> Vec<String> {
    let contents = fs::read_to_string(fp)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", fp, e));
    contents
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// 将 ESL 与 SKU 组合，并每 1000 条记录一批，发送 POST 请求进行绑定
fn store_bind_new(store: &str) -> Result<(), Box<dyn Error>> {
    let url = "http://172.16.127.37:8717/prismart/integration";
    // 读取 ESL 与 SKU 列表（假定两个文件行数相同）
    let esl_list = get_esl("esl.txt");
    let sku_list = get_esl("sku.txt");

    // 构造基础 JSON 数据
    let mut no_change = json!({
        "customerStoreCode": "wumart",
        "storeCode": store,
        "batchNo": format!("ps_{}", store)
    });

    let mut items_list: Vec<Value> = Vec::new();
    let mut round_n = 0;
    let client = Client::new();

    // 遍历 ESL 列表，使用索引取对应的 SKU
    for (i, e) in esl_list.iter().enumerate() {
        // 如果 sku_list 长度不足，则用空字符串
        let sku = sku_list.get(i).unwrap_or(&"".to_string());
        // 构造单条记录
        let change = json!({
            "sku": sku,
            "IIS_COMMAND": "BIND",
            "eslId": e,
            "customerStoreCode": "wumart",
            "storeCode": "5"
        });
        round_n += 1;
        items_list.push(change);

        if round_n > 999 {
            // 批量提交
            no_change["items"] = json!(items_list);
            // 生成随机批次号
            let batch_no = format!("bind_{}", rand::thread_rng().gen_range(1..1000));
            no_change["batchNo"] = json!(batch_no);
            let resp = client.post(url).json(&no_change).send()?;
            println!("异步返回参数:={}", resp.text()?);
            // 清空当前批次
            items_list.clear();
            round_n = 0;
        }
    }
    // 最后一批不足 1000 条时提交
    if !items_list.is_empty() {
        no_change["items"] = json!(items_list);
        let batch_no = format!("bind_{}", rand::thread_rng().gen_range(1..1000));
        no_change["batchNo"] = json!(batch_no);
        let resp = client.post(url).json(&no_change).send()?;
        println!("last request :={}", resp.text()?);
    }
    Ok(())
}

