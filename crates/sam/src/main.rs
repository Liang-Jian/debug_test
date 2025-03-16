
use anyhow_ext::{Result,Error};
use chrono::{Local, Datelike};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fs::{self, OpenOptions}, io::{Write, BufReader, BufRead}, time::Duration};
use tokio::{task, time::sleep};
use calamine::{open_workbook, Reader, Xlsx};


#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    status: u16,
    msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct InnerResponse {
    version: String,
    online: bool,
    up_time: String,
    up_time_sec: u64,
    cpu_free: u8,
    mem_free: u64,
    mem_total: u64,
    disk_use: u64,
    disk_total: u64,
    disk_life: String,
    net_send: u64,
    net_recv: u64,
    last_upgraded_ts: u64,
}

const API_URL: &str = "https://api.example.com/get_mac_data"; // 修改为你的 API 地址

const HAPP: &str = "https://10.11.20.55:9900";

// /api/cluster/devices/{m}/rpc/sysctrl/container/status
fn read_store() -> Vec<String> {
    // 读取整个文件内容，
    let contents =
        fs::read_to_string("D:\\forever\\src\\store.txt").expect("file to read store txt");
    // let contents = fs::read_to_string("D:\\forever\\src\\esl.txt").expect("file to read store txt");

    // 按行分割，并去除首尾空白，过滤掉空行
    let vec: Vec<String> = contents
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    // println!("{:?}", vec);
    vec
}

pub fn get_search_mac() -> Vec<String> {
    // 指定要读取的 XLSX 文件路径
    let path = "D:\\export.xlsx";
    let need_search_store = read_store();
    // 打开 xlsx 文件，返回 Xlsx 类型的工作簿对象
    let mut workbook: Xlsx<_> = open_workbook(path).expect("fail to read export.xlsx");
    let mut mac_list = Vec::new();
    // 尝试读取名称为 "APs" 的工作表
    if let Some(Ok(range)) = workbook.worksheet_range("APs") {
        // 遍历工作表中的每一行
        for row in range.rows() {
            // 假定该行至少包含 3 列数据
            if row.len() >= 3 {
                // 将对应单元格转换为字符串进行比较
                let customer_code = row[1].to_string();
                let store_code = row[2].to_string();
                if customer_code == "god" && need_search_store.contains(&store_code) {
                    mac_list.push(row[0].to_string().to_uppercase());
                }
            }
        }
    } else {
        println!("fail to reload APs table");
    }
    println!("{:?}", mac_list);
    mac_list
}

#[tokio::main]
async fn main() {
    loop {
        let now = Local::now();
        let next_run = now.date_naive().and_hms_opt(12, 0, 0).unwrap();
        let duration_until_next_run = next_run.signed_duration_since(now.naive_local()).to_std().unwrap_or(Duration::from_secs(0));

        println!("等待直到 12:00 运行...");
        sleep(duration_until_next_run).await;

        if let Err(e) = process_api().await {
            eprintln!("执行过程中出错: {}", e);
        }

        println!("任务完成，等待明天...");
        sleep(Duration::from_secs(24 * 60 * 60)).await;
    }
}


// async fn fetch_uptime_sec() -> Result<(), Box<dyn std::error::Error>> {
//     let happ = format!("{HAPP}/api/cluster/devices/{m}/rpc/sysctrl/container/status");
//     let client = Client::new();
//     let response: ApiResponse = client.get(API_URL).send().await?.json().await?;

//     if response.status != 200 {
//         return Err(format!("API 返回错误状态码: {}", response.status).into());
//     }

//     let parsed_msg: InnerResponse = serde_json::from_str(&response.msg)?;

//     let mac_address = "AA:BB:CC:DD:EE:FF"; // 这里改成实际的 MAC 地址
//     let uptime_sec = parsed_msg.up_time_sec;

//     let mut data: HashMap<String, u64> = load_existing_data(FILE_PATH);
//     data.insert(mac_address.to_string(), uptime_sec);

//     save_data_to_file(FILE_PATH, &data)?;

//     println!("MAC: {}, uptime_sec: {}", mac_address, uptime_sec);
//     Ok(())
// }

async fn process_api() -> Result<(), anyhow_ext::Error> {
    let today = Local::now().format("%Y-%m-%d").to_string();
    let yesterday = Local::now().checked_sub_days(chrono::Days::new(1)).unwrap().format("%Y-%m-%d").to_string();
    let mac_list = get_search_mac();

    let client = Client::new();
    for m in mac_list {
        let happ = format!("{HAPP}/api/cluster/devices/{m}/rpc/sysctrl/container/status");
        // let response = client.get(API_URL).send().await?.json::<Vec<ApiResponse>>().await?;
        let response: ApiResponse = client.get(API_URL).send().await?.json().await?;
        if response.status != 200 {
            sleep(Duration::from_secs(1)).await;
            return Err(anyhow_ext::Error::msg("API 返回错误状态码,{{response.status}"));
        }
        let parsed_msg: InnerResponse = serde_json::from_str(&response.msg)?;
        let uptime_sec = parsed_msg.up_time_sec;
        let mut data: HashMap<String, u64> = load_existing_data(FILE_PATH);
        data.insert(mac_address.to_string(), uptime_sec);
    }
        
    // let today_file = format!("data_{}.txt", today);
    // let yesterday_file = format!("data_{}.txt", yesterday);
    

    
    // save_data_to_file(&today_file, &response)?;

    // if let Ok(previous_data) = load_previous_data(&yesterday_file) {
    //     compare_uptime(&previous_data, &response)?;
    // }

    Ok(())
}

fn save_data_to_file(filename: &str, data: &[ApiResponse]) -> Result<()> {
    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(filename)?;

    for entry in data {
        writeln!(file, "{}", serde_json::to_string(entry)?)?;
    }
    
    println!("数据已保存到 {}", filename);
    Ok(())
}

fn load_previous_data(filename: &str) -> std::io::Result<HashMap<String, ApiResponse>> {
    let file = OpenOptions::new().read(true).open(filename)?;
    let reader = BufReader::new(file);

    let mut data_map = HashMap::new();
    for line in reader.lines() {
        if let Ok(json_line) = line {
            if let Ok(entry) = serde_json::from_str::<ApiResponse>(&json_line) {
                data_map.insert(entry.mac.clone(), entry);
            }
        }
    }

    Ok(data_map)
}

fn compare_uptime(old_data: &HashMap<String, ApiResponse>, new_data: &[ApiResponse]) -> std::io::Result<()> {
    let mut result_file = OpenOptions::new().create(true).append(true).open("result.txt")?;

    for entry in new_data {
        if let Some(prev_entry) = old_data.get(&entry.mac) {
            if entry.uptime < prev_entry.uptime {
                let log = format!(
                    "[{}] 设备 {} uptime 变小 ({} -> {}), 可能发生重启",
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    entry.mac,
                    prev_entry.uptime,
                    entry.uptime
                );
                println!("{}", log);
                writeln!(result_file, "{}", log)?;
            }
        }
    }

    Ok(())
}
