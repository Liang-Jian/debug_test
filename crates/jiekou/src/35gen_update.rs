use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::collections::HashSet;
use std::error::Error;
use chrono::{NaiveDateTime, Duration};

/// **日志切割**: 根据时间范围提取日志
fn log_cut(start_time: &str, end_time: &str) -> Result<(), Box<dyn Error>> {
    let input_file = "eslworking-pda.log";
    let output_file = "eslworking-pda-cut.log";
    
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);
    
    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_file)?;

    for line in reader.lines() {
        let line = line?;
        if line.len() >= 23 {
            let timestamp = &line[11..23];
            if timestamp >= start_time && timestamp <= end_time && &line[..4] == "2023" {
                writeln!(output, "{}", line)?;
            }
        }
    }
    println!("日志切割完成: {}", output_file);
    Ok(())
}

/// **计算时间差**
fn get_time_diff(start: &str, end: &str) -> Duration {
    let fmt = "%Y-%m-%d %H:%M:%S%.3f";
    let t1 = NaiveDateTime::parse_from_str(start, fmt).unwrap();
    let t2 = NaiveDateTime::parse_from_str(end, fmt).unwrap();
    t2 - t1
}

/// **读取 ESL ID 文件**
fn read_esl_ids(file_name: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let esl_ids: HashSet<String> = reader.lines().filter_map(Result::ok).collect();
    Ok(esl_ids)
}

/// **日志分析**
fn analyze_logs(esl_ids: &HashSet<String>, esl_list: &str) -> Result<(), Box<dyn Error>> {
    let log_file = "eslworking-pda-cut.log";
    let output_file = format!("{}.csv", esl_list);
    
    let file = File::open(log_file)?;
    let reader = BufReader::new(file);
    let mut csv_writer = csv::Writer::from_path(output_file)?;

    // CSV 头
    csv_writer.write_record(&[
        "价签更新开始时间", "价签更新结束时间", "更新花费时间", "更新成功率", "价签总量",
        "receive", "finish", "90%的价签完成时间", "99%的价签完成时间", "100%的价签完成时间", "异常价签"
    ])?;

    let mut start_times = Vec::new();
    let mut finish_times = Vec::new();
    let mut failed_esls = Vec::new();
    let mut finish_success = 0;

    for line in reader.lines() {
        let line = line?;
        
        if let Some(cap) = line.find("category=esl,action=receive,user_code=wumart.001,eslid=") {
            let esl = &line[cap + 51..cap + 63];
            if esl_ids.contains(esl) {
                start_times.push(line[..23].to_string());
            }
        }

        if let Some(cap) = line.find("category=esl,action=ack_result,user_code=wumart.001,eslid=") {
            let esl = &line[cap + 51..cap + 63];
            if esl_ids.contains(esl) {
                let ack_value = &line[cap + 97..cap + 99];
                if ack_value != "64" {
                    failed_esls.push(esl.to_string());
                }
            }
        }

        if let Some(cap) = line.find("esl_update_finished,user_code=wumart.001,eslid=") {
            let esl = &line[cap + 44..cap + 56];
            if esl_ids.contains(esl) {
                finish_times.push(line[..23].to_string());
                if line.contains("status=online") {
                    finish_success += 1;
                }
            }
        }
    }

    // 计算数据
    let total_esls = esl_ids.len();
    let receive_count = start_times.len();
    let finish_count = finish_times.len();
    
    if receive_count > 0 && finish_count > 0 {
        let start_time = &start_times[0];
        let end_time = &finish_times[finish_count - 1];

        let update_time = get_time_diff(start_time, end_time);
        let success_rate = (finish_success as f64 / total_esls as f64) * 100.0;

        let time_90 = get_time_diff(start_time, &finish_times[(total_esls as f64 * 0.9) as usize]);
        let time_99 = get_time_diff(start_time, &finish_times[(total_esls as f64 * 0.99) as usize]);
        let time_100 = get_time_diff(start_time, end_time);

        println!("价签更新开始时间: {}", start_time);
        println!("价签更新结束时间: {}", end_time);
        println!("更新花费时间: {}", update_time);
        println!("更新成功率: {:.2}%", success_rate);
        println!("价签总量: {}", total_esls);
        println!("receive: {}", receive_count);
        println!("finish: {}", finish_count);
        println!("90%的价签完成时间: {}", time_90);
        println!("99%的价签完成时间: {}", time_99);
        println!("100%的价签完成时间: {}", time_100);
        println!("异常价签: {:?}", failed_esls);

        // 写入 CSV
        csv_writer.write_record(&[
            start_time, end_time, &update_time.to_string(),
            &format!("{:.2}%", success_rate), &total_esls.to_string(),
            &receive_count.to_string(), &finish_count.to_string(),
            &time_90.to_string(), &time_99.to_string(), &time_100.to_string(),
            &format!("{:?}", failed_esls)
        ])?;
    }

    println!("分析完成，结果存入 {}.csv", esl_list);
    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    let start_time = "00:15";
    let end_time = "01:25";

    println!("请输入 ESL ID 文件名:");
    let mut esl_list = String::new();
    std::io::stdin().read_line(&mut esl_list)?;
    let esl_list = esl_list.trim(); // 移除换行符

    let esl_ids = read_esl_ids(esl_list)?;

    log_cut(start_time, end_time)?;
    analyze_logs(&esl_ids, esl_list)?;

    Ok(())
}
