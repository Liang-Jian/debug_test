// 检查数据库中是否有组网重复问题

use mongodb::{
    bson::{doc, Document},
    Client,
};
use futures::stream::TryStreamExt;
use std::collections::{HashMap, HashSet};
use std::error::Error;

const CI: &str = "mongodb://172.17.120.26:27017/";
const DB_NAME: &str = "esl6ceng4ap133";
const COLLECT: &str = "esl";

/// 统计 list 中各元素出现的次数，并返回出现次数大于1的元素及其计数
fn calc_count(list: &[String]) -> Vec<(String, i32)> {
    let mut counter = HashMap::new();
    for item in list {
        *counter.entry(item.clone()).or_insert(0) += 1;
    }
    counter
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .collect()
}

/// 根据给定 set_id, group_id, subnet 查询集合中符合条件的文档，返回它们的 eslId 列表
async fn insert_db(set_id: &str, group_id: i32, subnet: i32) -> Result<Vec<String>, Box<dyn Error>> {
    let client = Client::with_uri_str(CI).await?;
    let db = client.database(DB_NAME);
    let collection = db.collection::<Document>(COLLECT);

    let filter = doc! {
        "setId": set_id,
        "groupId": group_id,
        "subnet": subnet,
    };

    let mut cursor = collection.find(filter, None).await?;
    let mut esl_list = Vec::new();
    while let Some(doc) = cursor.try_next().await? {
        if let Ok(esl_id) = doc.get_str("eslId") {
            esl_list.push(esl_id.to_string());
        }
    }
    Ok(esl_list)
}

/// 查询所有文档，统计 binding.status == 2 的记录，并打印重复的资源情况
async fn is_repeat_status() -> Result<(), Box<dyn Error>> {
    let client = Client::with_uri_str(CI).await?;
    let db = client.database(DB_NAME);
    let collection = db.collection::<Document>(COLLECT);

    let mut ele_list: Vec<String> = Vec::new();

    let mut cursor = collection.find(doc! {}, None).await?;
    while let Some(doc) = cursor.try_next().await? {
        // 获取 eslId、setId、groupId 和 subnet
        let set_id = doc.get_str("setId").unwrap_or("");
        let group_id = doc.get_i32("groupId").unwrap_or(0);
        let subnet = doc.get_i32("subnet").unwrap_or(0);

        // 尝试获取 binding.status，如果 status == 2 则构造资源标识
        if let Some(binding) = doc.get_document("binding").ok() {
            if let Ok(status) = binding.get_i32("status") {
                if status == 2 {
                    let ele = format!("{}|{}|{}", set_id, group_id, subnet);
                    ele_list.push(ele);
                }
            }
        }
    }

    println!("总共 esl 个数: {}", ele_list.len());
    let unique: HashSet<_> = ele_list.iter().cloned().collect();
    println!("总共资源个数: {}", unique.len());

    let no_repeat_list = calc_count(&ele_list);
    println!("重复资源: {:?}", no_repeat_list);

    // 对于每个重复的资源，调用 insert_db 查询符合条件的 eslId 列表
    for (ele, count) in no_repeat_list {
        let parts: Vec<&str> = ele.split('|').collect();
        if parts.len() < 3 {
            continue;
        }
        let set_id = parts[0];
        let group_id: i32 = parts[1].parse().unwrap_or(0);
        let subnet: i32 = parts[2].parse().unwrap_or(0);
        let esl_list = insert_db(set_id, group_id, subnet).await?;
        println!(
            "setId={}, groupId={}, subnet={}, esl: {:?}",
            set_id, group_id, subnet, esl_list
        );
    }

    Ok(())
}


