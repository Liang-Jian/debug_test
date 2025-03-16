


use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

/// **按时间拆分日志**
fn split_log(path: &str, start: &str, end: &str, name: &str) -> io::Result<()> {
    let input_file = File::open(path)?;
    let reader = BufReader::new(input_file);

    let new_path = Path::new(path).parent().unwrap().join(format!("{}.log", name));
    let mut output_file = OpenOptions::new().create(true).append(true).open(&new_path)?;

    let mut is_recording = false;
    
    for line in reader.lines() {
        let line = line?;
        if line.contains(start) {
            is_recording = true;
        }
        if is_recording {
            writeln!(output_file, "{}", line)?;
        }
        if line.contains(end) {
            break;
        }
    }
    
    println!("日志已拆分，存储于: {:?}", new_path);
    Ok(())
}

/// **合并两个日志**
fn merge_log(path1: &str, path2: &str, name: &str) -> io::Result<()> {
    let file1 = File::open(path1)?;
    let file2 = File::open(path2)?;

    let reader1 = BufReader::new(file1);
    let reader2 = BufReader::new(file2);

    let new_path = Path::new(path1).parent().unwrap().join(format!("{}.txt", name));
    let mut output_file = OpenOptions::new().create(true).append(true).open(&new_path)?;

    for line in reader1.lines() {
        writeln!(output_file, "{}", line?)?;
    }
    for line in reader2.lines() {
        writeln!(output_file, "{}", line?)?;
    }

    println!("日志已合并，存储于: {:?}", new_path);
    Ok(())
}

fn run() -> io::Result<()> {
    let log_path = r"D:\big\eslworking.log";
    let start_time = "2024-03-13 07:28";
    let end_time = "2024-03-13 23:58";
    let split_name = "bak";

    split_log(log_path, start_time, end_time, split_name)?;

    // **如果需要合并两个日志**
    // merge_log("log1.txt", "log2.txt", "merged")?;

    Ok(())
}
