

use serialport::prelude::*;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use std::thread;
use log::{info, error};
use fern::Dispatch;

// **初始化日志**
fn init_logger() {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ));
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file("serial_log.log").unwrap())
        .apply()
        .unwrap();
}

// **串口读取函数**
fn serial_recv(port_name: &str, baud_rate: u32) {
    let settings = SerialPortSettings {
        baud_rate,
        timeout: Duration::from_secs(10),
        ..Default::default()
    };

    match serialport::open_with_settings(port_name, &settings) {
        Ok(mut port) => {
            let mut buffer: Vec<u8> = vec![0; 1024];
            loop {
                match port.read(buffer.as_mut_slice()) {
                    Ok(bytes_read) => {
                        let data = String::from_utf8_lossy(&buffer[..bytes_read]);
                        info!("{} received: {}", port_name, data);
                    }
                    Err(e) => {
                        error!("Error reading from {}: {:?}", port_name, e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            error!("Failed to open {}: {:?}", port_name, e);
        }
    }
}

// **多线程任务**
fn run_threads(serial_ports: Vec<String>, baud_rate: u32) {
    serial_ports.par_iter().for_each(|port| {
        serial_recv(port, baud_rate);
    });
}

// **多进程任务**
fn run_processes(serial_ports: Vec<String>, baud_rate: u32) {
    let ports_split: Vec<Vec<String>> = serial_ports.chunks(2).map(|s| s.to_vec()).collect();
    
    ports_split.into_par_iter().for_each(|port_group| {
        let port_group = port_group.clone();
        thread::spawn(move || {
            run_threads(port_group, baud_rate);
        });
    });
}

fn main() {
    init_logger();
    let serial_ports = vec!["/dev/ttyUSB0".to_string(), "/dev/ttyUSB1".to_string(), "/dev/ttyUSB2".to_string(), "/dev/ttyUSB3".to_string()];
    let baud_rate = 115200;

    run_processes(serial_ports, baud_rate);
}
