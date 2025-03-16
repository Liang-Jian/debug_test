

use std::net::TcpStream;
use std::time::Duration;

fn main() {
    let ip_range = "10.12.63.";
    let port_list = [22, 80, 443, 3306];

    for i in 1..=100 {
        let ip = format!("{}{}", ip_range, i);
        for port in port_list.iter() {
            let addr = format!("{}:{}", ip, port);
            // 尝试在 1 秒内建立连接
            if TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_secs(1)).is_ok() {
                println!("IP {} 的 {} 端口开放", ip, port);
            }
        }
    }
}
