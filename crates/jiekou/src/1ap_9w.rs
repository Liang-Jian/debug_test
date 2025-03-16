

use chrono::{DateTime, Utc};
use mongodb::{bson::doc, options::ClientOptions, sync::Client};
use rand::Rng;

/// MongoDB 连接参数
const CI: &str = "mongodb://172.17.120.26:27017/";
const DB: &str = "esl4cengyace3";
const COLLECTION: &str = "esl";

/// ESL ID 生成范围
const ESL_ID_ESL_OCT: (i64, i64) = (1392508928, 360777252863);

/// 设置 ID 前缀
const SET_ID_F: i32 = 4000;
const SET_ID_E: &str = "-00-66";

/// 生成 ESL ID
fn esl_fix(esl: i64) -> String {
    let esl_str = format!("{:08X}", esl); // 转为 8 位十六进制
    format!(
        "{}-{}-{}-{}",
        &esl_str[0..2], &esl_str[2..4], &esl_str[4..6], &esl_str[6..8]
    )
}

/// 生成 Set ID
fn set_fix(set_id_f: i32) -> String {
    let set_str = format!("{:04}", set_id_f);
    format!("{}-{}", &set_str[0..2], &set_str[2..])
}

/// 连接 MongoDB
fn connect_mongodb() -> mongodb::sync::Collection<mongodb::bson::Document> {
    let client_options = ClientOptions::parse(CI).expect("Failed to parse MongoDB URI");
    let client = Client::with_options(client_options).expect("Failed to connect to MongoDB");
    let database = client.database(DB);
    database.collection(COLLECTION)
}

/// 生成 ESL 数据并插入数据库
fn make_100_esl() {
    let collection = connect_mongodb();
    let mut rng = rand::thread_rng();
    let mut esl_id = ESL_ID_ESL_OCT.0;

    let all_mac = vec!["98:6D:35:76:6D:AC"];
    let create_date: DateTime<Utc> = Utc::now();

    for mac in &all_mac {
        for _s in 0..16 {
            let set_id = format!("{}{}", set_fix(SET_ID_F + _s), SET_ID_E);
            for group_id in 1..=23 {
                for subnet in 0..256 {
                    let esl = esl_fix(esl_id);
                    esl_id += 1;

                    let doc = doc! {
                        "eslId": &esl,
                        "apMac": mac,
                        "apOffset": 2,
                        "attribute": 0,
                        "battery": 2.5,
                        "batteryType": 0,
                        "cipherMode": 0,
                        "cipherStatus": 0,
                        "commKey": "",
                        "crc": 0,
                        "createDate": create_date.to_rfc3339(),
                        "dataChannel": 220,
                        "direction": 0,
                        "driverVersion": 14,
                        "eslInfo": 0,
                        "extEslid": "857940081427060039",
                        "extraAttribute": doc! {
                            "aoa": false,
                            "bid": false,
                            "eslInfoG4": "01DC00",
                            "maxPage": 13,
                            "move": false,
                            "secretKeyType": "USER_ENCRYPT"
                        },
                        "firmware": 8410277,
                        "groupChannel": 220,
                        "groupId": group_id,
                        "netId": 131,
                        "notActive": 0,
                        "notNetlink": 0,
                        "productId": 1420116768,
                        "rom": 14,
                        "setChannel": 220,
                        "setId": &set_id,
                        "setWor": 4,
                        "status": 1,
                        "subnet": subnet,
                        "subnetId": 4,
                        "synced": true,
                        "updateDate": create_date.to_rfc3339(),
                        "userCode": "default",
                        "version": 16,
                        "binding": doc! {
                            "apMac": mac,
                            "dataChannel": 220,
                            "eslId": &esl,
                            "groupChannel": 220,
                            "groupId": group_id,
                            "manual": 0,
                            "roamingApMac": "",
                            "setChannel": 220,
                            "setId": &set_id,
                            "startTime": create_date.to_rfc3339(),
                            "status": 2,
                            "subnet": subnet
                        }
                    };

                    collection.insert_one(doc, None).expect("Failed to insert document");
                }
            }
        }
    }
    println!("插入完成！");
}

fn main() {
    make_100_esl();
}
