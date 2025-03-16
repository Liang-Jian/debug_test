use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::net::{TcpStream};
use std::time::{Duration, Instant};
use std::thread::sleep;
use std::process::Command;
use std::fmt;
use std::str;
use std::sync::mpsc;
use crc::{Crc, CRC_16_CCITT_FALSE}; // Add the `crc` crate to calculate CRC16

#[derive(Debug)]
pub struct ESL {
    esl_id: String,
    task_id: String,
    status: String,
}

fn get_firmware(product: &str) -> u64 {
    let firmware = &product[14..16]
        .to_string() + &product[12..14] + &product[10..12] + &product[8..10]
        + &product[6..8] + &product[4..6];
    u64::from_str_radix(&firmware, 16).unwrap()
}

const CRC16_TAB: [u16; 256] = [
    0x0000, 0x1021, 0x2042, 0x3063, 0x4084, 0x50a5, 0x60c6, 0x70e7,
    0x8108, 0x9129, 0xa14a, 0xb16b, 0xc18c, 0xd1ad, 0xe1ce, 0xf1ef,
    0x1231, 0x0210, 0x3273, 0x2252, 0x52b5, 0x4294, 0x72f7, 0x62d6,
    0x9339, 0x8318, 0xb37b, 0xa35a, 0xd3bd, 0xc39c, 0xf3ff, 0xe3de,
    0x2462, 0x3443, 0x0420, 0x1401, 0x64e6, 0x74c7, 0x44a4, 0x5485,
    0xa56a, 0xb54b, 0x8528, 0x9509, 0xe5ee, 0xf5cf, 0xc5ac, 0xd58d,
    0x3653, 0x2672, 0x1611, 0x0630, 0x76d7, 0x66f6, 0x5695, 0x46b4,
    0xb75b, 0xa77a, 0x9719, 0x8738, 0xf7df, 0xe7fe, 0xd79d, 0xc7bc,
    0x48c4, 0x58e5, 0x6886, 0x78a7, 0x0840, 0x1861, 0x2802, 0x3823,
    0xc9cc, 0xd9ed, 0xe98e, 0xf9af, 0x8948, 0x9969, 0xa90a, 0xb92b,
    0x5af5, 0x4ad4, 0x7ab7, 0x6a96, 0x1a71, 0x0a50, 0x3a33, 0x2a12,
    0xdbfd, 0xcbdc, 0xfbbf, 0xeb9e, 0x9b79, 0x8b58, 0xbb3b, 0xab1a,
    0x6ca6, 0x7c87, 0x4ce4, 0x5cc5, 0x2c22, 0x3c03, 0x0c60, 0x1c41,
    0xedae, 0xfd8f, 0xcdec, 0xddcd, 0xad2a, 0xbd0b, 0x8d68, 0x9d49,
    0x7e97, 0x6eb6, 0x5ed5, 0x4ef4, 0x3e13, 0x2e32, 0x1e51, 0x0e70,
    0xff9f, 0xefbe, 0xdfdd, 0xcffc, 0xbf1b, 0xaf3a, 0x9f59, 0x8f78,
];

fn str_to_uchar_crc(crc: u16, string: &str) -> u16 {
    let bytes = hex::decode(string).unwrap();
    let len = bytes.len();
    let mut crc = crc;
    for i in 0..len {
        let byte = bytes[i];
        crc = (crc << 8 ^ CRC16_TAB[((crc >> 8) ^ byte) as usize]) & 0xFFFF;
    }
    crc
}

fn get_ap_location(ip: &str) -> String {
    let path = "/root/.jenkins/workspace/定时分析基站日志reset/config/ap_list.txt";
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut ap_info: HashMap<String, String> = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 {
            ap_info.insert(parts[0].to_string(), parts[1].to_string());
        }
    }
    ap_info.get(ip).cloned().unwrap_or_else(|| "未知位置".to_string())
}

fn main() {
    let product = "0a402005008015b0";
    let firmware = get_firmware(product);
    println!("Firmware version: {}", firmware);

    let ap_location = get_ap_location("192.168.118.101");
    println!("AP location: {}", ap_location);
}
