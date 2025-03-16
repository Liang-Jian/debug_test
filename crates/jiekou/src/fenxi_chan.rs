
use flate2::read::GzDecoder;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

/// 解析 Gzip 文件，获取所有的 `ap_mac`
fn get_mac(fp: &str) -> Vec<String> {
    let file = File::open(fp).expect("Failed to open Gzip file");
    let decoder = GzDecoder::new(file);
    let reader = BufReader::new(decoder);

    let mut mac_id: HashSet<String> = HashSet::new();
    let mac_regex = Regex::new(r",ap_mac=(.*?),").unwrap();

    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(capture) = mac_regex.captures(&line) {
                mac_id.insert(capture[1].to_string());
            }
        }
    }

    let mac_list: Vec<String> = mac_id.into_iter().collect();
    println!("所有的 MAC ID: {:?}", mac_list);
    mac_list
}

/// 格式化输出
fn format_output(mac_id: &str, log_data: &Vec<Vec<String>>) {
    println!("MAC ID: {}", mac_id);
    for (i, entries) in log_data.iter().enumerate() {
        if !entries.is_empty() {
            println!("\t{}: {} 条记录", i, entries.len());
        }
    }
}

/// 修正数据，去重
fn fix_data(log_data: &mut Vec<Vec<String>>) {
    for entries in log_data.iter_mut() {
        let unique_entries: HashSet<String> = entries.iter().cloned().collect();
        *entries = unique_entries.into_iter().collect();
    }
}

/// 背景扫描数据分析
fn bgscan(fp: &str) {
    let mac_list = get_mac(fp);
    let file = File::open(fp).expect("Failed to open Gzip file");
    let decoder = GzDecoder::new(file);
    let reader = BufReader::new(decoder);

    let time_regex = Regex::new(r"(\d{1,2}):\d{1,2}:\d{1,2}\.?\d{0,3} I").unwrap();

    for mac_id in mac_list.iter() {
        let mut logs_per_hour: Vec<Vec<String>> = vec![vec![]; 25];

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.contains(&format!(",action=scan,ap_mac={}", mac_id)) {
                    if let Some(capture) = time_regex.captures(&line) {
                        let time_str = capture[1].to_string();
                        if let Ok(hour) = time_str.parse::<usize>() {
                            logs_per_hour[hour].push(time_str.clone());
                        }
                    }
                }
            }
        }

        fix_data(&mut logs_per_hour);
        format_output(mac_id, &logs_per_hour);
    }
}

fn main() {
    let filepath = r"C:\Users\chengtao.shi\Desktop\log\channel_2022-04-19-1.log.gz";
    bgscan(filepath);
}
