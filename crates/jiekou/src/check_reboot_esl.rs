
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use flate2::read::GzDecoder;
use regex::Regex;

fn get_all_files(dir_path: &str) -> Vec<String> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.path().file_name() {
                    if let Some(name_str) = file_name.to_str() {
                        files.push(name_str.to_string());
                    }
                }
            }
        }
    }
    println!("Found files: {:?}", files);
    files
}

fn find_reboot_esl(file_path: &str) -> Vec<String> {
    let mut reboot_esl = Vec::new();

    let file = File::open(file_path).expect("无法打开文件");
    let decoder = GzDecoder::new(file);
    let reader = BufReader::new(decoder);

    let regex = Regex::new(r"eslid=(.*?),").unwrap();

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.contains("type=V4HB,eslid=") && line.contains("ap_offset=0,") {
                if let Some(captures) = regex.captures(&line) {
                    if let Some(esl) = captures.get(1) {
                        let esl_id = esl.as_str().to_string();
                        if !reboot_esl.contains(&esl_id) {
                            reboot_esl.push(esl_id);
                        }
                    }
                }
            }
        }
    }
    println!("File: {:?} -> {:?}", file_path, reboot_esl);
    reboot_esl
}

fn count_occurrences(esl_list: &[String]) -> HashMap<String, usize> {
    let mut counter = HashMap::new();
    for esl in esl_list {
        *counter.entry(esl.clone()).or_insert(0) += 1;
    }
    counter
}

fn run() {
    let dir_path = "D:/bbit_round2/eslw_v5-5.0.1rc7/log/2023-02";
    let mut all_esl = Vec::new();

    for file_name in get_all_files(dir_path) {
        let file_path = format!("{}/{}", dir_path, file_name);
        all_esl.extend(find_reboot_esl(&file_path));
    }

    let esl_count = count_occurrences(&all_esl);
    let mut sorted_counts: Vec<(&String, &usize)> = esl_count.iter().collect();
    sorted_counts.sort_by(|a, b| b.1.cmp(a.1));

    println!("\n出现最多的 ESL 设备:");
    for (esl, count) in sorted_counts.iter().take(10) {
        println!("ESL: {}，出现次数: {}", esl, count);
    }
}

fn main() {
    run();
}
