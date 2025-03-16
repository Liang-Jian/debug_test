

use mongodb::{Client, bson::doc};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufReader};
use std::collections::HashSet;
use tokio;

#[derive(Debug, Serialize, Deserialize)]
struct Binding {
    status: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ESL {
    eslId: String,
    rom: i32,
    binding: Option<Binding>,
}

async fn upgrade_esl(mongo_uri: &str, db_name: &str, collection_name: &str, white_list_path: &str) {
    // 读取白名单文件
    let esl_list = get_white_list(white_list_path);
    let client = Client::with_uri_str(mongo_uri).await.unwrap();
    let db = client.database(db_name);
    let collection = db.collection::<ESL>(collection_name);

    let mut count = 0;

    // 遍历 MongoDB 中的所有文档
    let cursor = collection.find(doc! { "rom": 46 }, None).await.unwrap();
    for result in cursor {
        match result {
            Ok(esl) => {
                if esl_list.contains(&esl.eslId) {
                    if let Some(binding) = esl.binding {
                        if let Some(status) = binding.status {
                            if status == 1 {
                                count += 1;
                            }
                        } else {
                            println!("{} has no status", esl.eslId);
                        }
                    } else {
                        println!("{} no binding", esl.eslId);
                    }
                }
            },
            Err(e) => println!("Error while processing document: {}", e),
        }
    }

    println!("Count of ESL with rom 46 and binding status 1: {}", count);
}

fn get_white_list(file_name: &str) -> HashSet<String> {
    let file = File::open(file_name).expect("Unable to open white list file");
    let reader = BufReader::new(file);
    let mut esl_id_list = HashSet::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let id = line.replace("=wumart.001", "").trim().to_string();
                esl_id_list.insert(id);
            },
            Err(e) => println!("Error reading line: {}", e),
        }
    }

    esl_id_list
}

#[tokio::main]
async fn run() {
    let mongo_uri = "mongodb://172.17.120.26:27017/";
    let db_name = "esl4ceng4ap2";
    let collection_name = "esl";
    let white_list_path = r"C:\Users\chengtao.shi\Desktop\4ceng_white_list.txt";

    upgrade_esl(mongo_uri, db_name, collection_name, white_list_path).await;
}
