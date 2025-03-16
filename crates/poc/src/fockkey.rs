

use enigo::*;
use std::{thread, time::Duration};

fn main() {
    let mut enigo = Enigo::new();
    let command = "/usr/local/bin/season web";

    // 3秒延迟，等待用户切换到目标窗口
    thread::sleep(Duration::from_secs(3));

    for i in 0..50 {
        // 模拟输入 `/usr/local/bin/season web`
        for c in command.chars() {
            if c == ' ' {
                enigo.key_click(Key::Space);
            } else {
                enigo.key_sequence(&c.to_string());
            }
            thread::sleep(Duration::from_millis(100)); // 确保每个字符都被正确输入
        }

        // 模拟按下 Enter 键
        enigo.key_click(Key::Return);

        // 等待 15 秒
        thread::sleep(Duration::from_secs(15));

        // 模拟按下 Ctrl+C
        enigo.key_down(Key::Control);
        enigo.key_click(Key::Layout('c'));
        enigo.key_up(Key::Control);

        println!("Run {}", i);
    }
}
