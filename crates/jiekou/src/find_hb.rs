use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use flate2::read::GzDecoder;
use regex::Regex;

/// 获取当前目录下所有 `.gz` 文件
fn get_all_gz_files() -> Vec<String> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "gz") {
                    if let Some(file_name) = path.to_str() {
                        files.push(file_name.to_string());
                    }
                }
            }
        }
    }
    files
}

/// 统计 heartbeat 日志中指定 ESL ID 的出现次数
fn count_esl_occurrences(file_path: &str, esl_id: &str) {
    if !Path::new(file_path).exists() {
        println!("文件不存在: {}", file_path);
        return;
    }

    let file = fs::File::open(file_path).expect("无法打开文件");
    let gz_decoder = GzDecoder::new(file);
    let reader = BufReader::new(gz_decoder);

    let mut esl_count = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            if line.contains("V4HB") && line.contains(esl_id) {
                println!("{}", line);
                esl_count += 1;
            }
        }
    }

    println!(
        "文件: {}; ESL ID: {}; 发现 {} 次",
        file_path, esl_id, esl_count
    );
}

/// 从 API 响应中提取电量信息
fn extract_battery_info(line: &str, esl_id: &str) -> Option<String> {
    let re = Regex::new(r#"esl_id'\s*:\s*'([^']+)'.*?battery'\s*:\s*([\d\.]+)"#).unwrap();
    for cap in re.captures_iter(line) {
        if &cap[1] == esl_id {
            return Some(cap[2].to_string());
        }
    }
    None
}

/// 解析 heartbeat API 日志，查找指定 ESL ID 的电量信息
fn find_battery_info(file_path: &str, esl_id: &str) {
    if !Path::new(file_path).exists() {
        println!("文件不存在: {}", file_path);
        return;
    }

    let file = fs::File::open(file_path).expect("无法打开文件");
    let gz_decoder = GzDecoder::new(file);
    let reader = BufReader::new(gz_decoder);

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.contains("battery=") && line.contains(esl_id) {
                if let Some(battery) = extract_battery_info(&line, esl_id) {
                    println!(
                        "文件: {}; ESL ID: {}; 电量: {}",
                        file_path, esl_id, battery
                    );
                    return;
                }
            }
        }
    }
}

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let esl_id = if args.len() > 1 { &args[1] } else { "30-55-5B-2E" };

//     println!("正在搜索 ESL ID: {}", esl_id);
//     for file in get_all_gz_files() {
//         count_esl_occurrences(&file, esl_id);
//         find_battery_info(&file, esl_id);
//     }
// }
