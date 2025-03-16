
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn bgscan(file_path: &str, target_mac: &str) {
    let file = File::open(file_path).expect("无法打开日志文件");
    let reader = BufReader::new(file);
    
    let mut bgscans: HashMap<String, Vec<u8>> = HashMap::new();
    
    for line in reader.lines() {
        let line = line.expect("无法读取行");
        
        if line.contains(target_mac) && line.contains("duty_cyle") {
            let rssi = extract_value(&line, "rssi=");
            let duty_cycle = extract_value(&line, "duty_cyle=");
            let channel = extract_channel(&line);

            let rssi: i32 = rssi.parse().unwrap_or(0);
            let duty_cycle: i32 = duty_cycle.parse().unwrap_or(0);

            let mut flag = 0; // 0: 无干扰, 1: 干扰
            if rssi < 60 || duty_cycle > 50 {
                flag = 1;
            }

            bgscans.entry(channel)
                .or_insert_with(Vec::new)
                .push(flag);
        }
    }

    println!("\n信道分析结果:");
    println!("---------------------------------------------------");
    println!("| 信道 | 无干扰次数 | 有干扰次数 | 总数 | 干扰占比 (%) |");
    println!("---------------------------------------------------");

    for (channel, flags) in &bgscans {
        let no_interference = flags.iter().filter(|&&x| x == 0).count();
        let interference = flags.len() - no_interference;
        let total = flags.len();
        let interference_ratio = (interference as f64 / total as f64) * 100.0;

        println!(
            "| {:>4} | {:>8} | {:>8} | {:>4} | {:>8.1} % |",
            channel, no_interference, interference, total, interference_ratio
        );
    }
    println!("---------------------------------------------------");
}

/// 提取 `key=value` 形式的值
fn extract_value(line: &str, key: &str) -> String {
    line.split(key)
        .nth(1)
        .unwrap_or("")
        .split(',')
        .next()
        .unwrap_or("")
        .to_string()
}

/// 提取 `channel` 值
fn extract_channel(line: &str) -> String {
    line.split("channel=")
        .nth(1)
        .unwrap_or("")
        .split(',')
        .next()
        .unwrap_or("")
        .to_string()
}

fn run() {
    let log_path = "D:/ESLW/eslworking5.0.2rc10/log/channel.log";
    let mac_address = "08:71:58:23:CF:10";

    bgscan(log_path, mac_address);
}
