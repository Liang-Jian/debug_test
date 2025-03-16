

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::path::Path;

/// 合并两个 gzip 日志文件
fn merge(fp: &str, fp1: &str, output_name: &str) -> std::io::Result<()> {
    // 解析路径，生成新文件路径
    let path = Path::new(fp);
    let parent_dir = path.parent().unwrap_or(Path::new("."));
    let new_filename = format!("{}/{}.log.gz", parent_dir.display(), output_name);

    // 打开 gzip 压缩的日志文件
    let file1 = File::open(fp)?;
    let file2 = File::open(fp1)?;
    
    let decoder1 = GzDecoder::new(file1);
    let decoder2 = GzDecoder::new(file2);

    let reader1 = BufReader::new(decoder1);
    let reader2 = BufReader::new(decoder2);

    // 创建新 gzip 文件
    let output_file = File::create(&new_filename)?;
    let mut encoder = GzEncoder::new(output_file, Compression::default());

    // 读取并写入第一个日志文件内容
    for line in reader1.lines() {
        writeln!(encoder, "{}", line?)?;
    }

    // 读取并写入第二个日志文件内容
    for line in reader2.lines() {
        writeln!(encoder, "{}", line?)?;
    }

    println!("合并完成: {}", new_filename);
    Ok(())
}

// fn main() {
//     let fp = "E:/test-X/heartbeat_2023-02-04-10.log.gz";
//     let fp1 = "E:/test-X/heartbeat_2023-02-04-11.log.gz";
//     let name = "merge";

//     if let Err(e) = merge(fp, fp1, name) {
//         eprintln!("错误: {}", e);
//     }
// }
