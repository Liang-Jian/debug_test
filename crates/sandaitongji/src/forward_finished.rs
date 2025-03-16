
use regex::Regex;
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufReader, BufRead, Write};
use plotters::prelude::*;

fn get_quickflash_time(file_name: &str, org_esls: &mut Vec<String>, time_windows: &Vec<f64>) -> Result<(), Box<dyn Error>> {
    // 用于存储 (eslid, work_time) 的列表
    let mut eslid_worktime_list: Vec<(String, u64)> = Vec::new();
    // 用于防止重复添加
    let mut eslid_list: Vec<String> = Vec::new();
    // 收到更新任务的价签
    let mut receive_esl_list: Vec<String> = Vec::new();
    // 存储工作时间超过阈值（6000000）的价签（后续排序使用）
    let mut long_time_esl_dic: Vec<(String, u64)> = Vec::new();
    let mut failed_eslid_list: Vec<String> = Vec::new();

    // 打开日志文件
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    // 准备正则表达式
    let re_success = Regex::new(r",eslid=(.*?),type=FORWARD,.*,work_time=(\d+)")?;
    let re_failed = Regex::new(r",eslid=(.*?),.*,ack_value=(\d+),")?;

    for line_result in reader.lines() {
        let line = line_result?;
        // 如果收到更新任务的日志行
        if line.contains("action=receive") && line.contains(",payload_type=FORWARD") {
            if let Some(part) = line.split(",eslid=").nth(1) {
                if let Some(esl_id) = part.split(',').next() {
                    let esl_id = esl_id.to_string();
                    if !receive_esl_list.contains(&esl_id) {
                        receive_esl_list.push(esl_id);
                    }
                }
            }
        }
        // 成功更新的日志行
        if line.contains("FORWARD") && line.contains(",result=SUCCESS,ack_value=64") {
            if let Some(caps) = re_success.captures(&line) {
                let _esl_id = caps.get(1).unwrap().as_str().to_string();
                let _work_time: u64 = caps.get(2).unwrap().as_str().parse()?;
                // 如果在 org_esls 中则移除
                if let Some(pos) = org_esls.iter().position(|x| *x == _esl_id) {
                    org_esls.remove(pos);
                }
                if _work_time > 6000000 {
                    long_time_esl_dic.push((_esl_id.clone(), _work_time));
                }
                if !eslid_list.contains(&_esl_id) {
                    eslid_list.push(_esl_id.clone());
                    eslid_worktime_list.push((_esl_id, _work_time));
                }
            }
        }
        // 失败的日志行
        if line.contains("FORWARD") && line.contains(",result=FAILED") {
            if let Some(caps) = re_failed.captures(&line) {
                let _failed_esl_id = caps.get(1).unwrap().as_str().to_string();
                let _failed_esl_ack = caps.get(2).unwrap().as_str();
                if let Some(pos) = org_esls.iter().position(|x| *x == _failed_esl_id) {
                    org_esls.remove(pos);
                }
                if !failed_eslid_list.contains(&_failed_esl_id) {
                    failed_eslid_list.push(_failed_esl_id.clone());
                    println!("失败价签：{}， ack_value:{}", _failed_esl_id, _failed_esl_ack);
                }
            }
        }
    } // end for

    println!("未收到更新任务的价签id为：");
    println!("{:?}", org_esls);
    println!("共有{}个价签收到更新任务,其中成功更新的数量为{}", receive_esl_list.len(), eslid_worktime_list.len());
    println!("超过120s的价签id为：");
    long_time_esl_dic.sort_by(|a, b| b.1.cmp(&a.1));
    for (key, value) in &long_time_esl_dic {
        println!("{}\t{}", key, value);
    }

    // 统计每个时间窗口内成功更新的价签数量
    let mut counts = vec![0; time_windows.len() + 1];
    for &(_, time) in &eslid_worktime_list {
        let mut matched = false;
        for (i, &window) in time_windows.iter().enumerate() {
            if i == 0 && time <= (window * 1000.0) as u64 {
                counts[i] += 1;
                matched = true;
                break;
            } else if i > 0 && time > (time_windows[i - 1] * 1000.0) as u64 && time <= (window * 1000.0) as u64 {
                counts[i] += 1;
                matched = true;
                break;
            }
        }
        if !matched {
            *counts.last_mut().unwrap() += 1;
        }
    }

    // 构造时间窗口标签
    let mut labels: Vec<String> = Vec::new();
    for (i, &window) in time_windows.iter().enumerate() {
        if i == 0 {
            labels.push(format!("0-{}s", window));
        } else {
            labels.push(format!("{}-{}s", time_windows[i - 1], window));
        }
    }
    labels.push(format!(">{}s", time_windows.last().unwrap()));

    for (label, count) in labels.iter().zip(counts.iter()) {
        println!("{}\t{}", label, count);
    }

    eslid_worktime_list.sort_by(|a, b| b.1.cmp(&a.1));
    println!("详细工作时间数据: {:?}", eslid_worktime_list);

    // 使用 plotters 绘制柱形图并保存
    let chart_path = "success_update_chart.png";
    {
        let drawing_area = BitMapBackend::new(chart_path, (800, 600)).into_drawing_area();
        drawing_area.fill(&WHITE)?;
        let max_count = *counts.iter().max().unwrap_or(&1);
        let mut chart = ChartBuilder::on(&drawing_area)
            .caption("不同时间窗口内成功更新价签数量统计", ("sans-serif", 20))
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(0..labels.len(), 0..(max_count + 5))?;

        chart.configure_mesh()
            .x_labels(labels.len())
            .x_label_formatter(&|idx| labels.get(*idx).unwrap_or(&"".to_string()).to_string())
            .y_desc("成功更新价签数量")
            .x_desc("时间窗口")
            .draw()?;

        // 绘制每个柱形图
        for (i, &count) in counts.iter().enumerate() {
            chart.draw_series(std::iter::once(Rectangle::new(
                [(i, 0), (i + 1, count)],
                BLUE.filled(),
            )))?;
            // 在柱形上方添加数字
            chart.draw_series(std::iter::once(Text::new(
                format!("{}", count),
                (i + 1, count),
                ("sans-serif", 15).into_font(),
            )))?;
        }
    }
    println!("柱形图已保存到: {}", chart_path);

    // 将 eslid_worktime_list 保存到文本文件中
    let txt_path = "eslid_worktime_list.txt";
    let mut file_out = File::create(txt_path)?;
    for (eslid, worktime) in &eslid_worktime_list {
        writeln!(file_out, "{}\t{}", eslid, worktime)?;
    }
    println!("eslid_worktime_list 已保存到: {}", txt_path);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // 读取日志文件，提取满足条件的唯一 eslid（取前 11 个字符，当第三个字符为 '-'）
    let log_file = "eslworking.log";
    let contents = fs::read_to_string(log_file)?;
    let mut eslids: Vec<String> = Vec::new();
    for line in contents.lines() {
        if line.len() >= 11 && line.chars().nth(2) == Some('-') {
            let candidate = &line[0..11];
            if !eslids.contains(&candidate.to_string()) {
                eslids.push(candidate.to_string());
            }
        }
    }

    // 从命令行获取时间窗口列表
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("请在命令行输入时间窗口列表，如 [5,10,20]");
        std::process::exit(1);
    }
    let windows_str = args[1].trim_matches(|c| c == '[' || c == ']');
    let time_windows: Vec<f64> = windows_str
        .split(',')
        .map(|s| s.trim().parse())
        .collect::<Result<Vec<f64>, _>>()
        .unwrap_or_else(|_| {
            eprintln!("输入的时间窗口列表格式不正确，请输入有效的数字列表，如 [5,10,20] 或 [5.2,10.5,20.7]");
            std::process::exit(1);
        });

    get_quickflash_time(log_file, &mut eslids, &time_windows)?;

    Ok(())
}
