


use headless_chrome::{Browser, LaunchOptions};
use sqlx::{MySql, MySqlPool, Row};
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use log::{info, error};
use std::{thread, time::Duration};

// 网站 URL
const URL: &str = "https://www.365-288.com/#/AC/B1/C1/D1002/E106068883/G938/H^1/";

// MySQL 配置
const DB_URL: &str = "mysql://username:password@localhost:3306/your_database";

// 邮件发送配置
const SMTP_SERVER: &str = "smtp.sina.com";
const EMAIL_USER: &str = "your_email@sina.com";
const EMAIL_PASS: &str = "your_password";
const RECEIVER_EMAIL: &str = "your_email@sina.com";

/// 获取比赛赔率
fn get_all_match_peilv() -> Vec<String> {
    let browser = Browser::new(LaunchOptions::default()).expect("Failed to launch browser");
    let tab = browser.new_tab().expect("Failed to open new tab");

    // 打开网页
    tab.navigate_to(URL).expect("Failed to navigate");
    thread::sleep(Duration::from_secs(10)); // 等待页面加载

    // 执行 JavaScript 获取赔率数据
    let js_script = r#"
        let oddsData = [];
        let elements = document.getElementsByClassName('src-ParticipantCenteredStacked48_Odds');
        for (let i = 0; i < elements.length; i++) {
            oddsData.push(elements[i].innerText);
        }
        oddsData;
    "#;

    match tab.evaluate(js_script, false) {
        Ok(result) => {
            if let Some(odds) = result.into_json::<Vec<String>>().ok() {
                info!("获取赔率数据成功: {:?}", odds);
                return odds;
            }
        }
        Err(err) => {
            error!("获取赔率数据失败: {:?}", err);
        }
    }
    vec![]
}

/// 查询 MySQL 历史数据
async fn query_mysql() -> Result<Vec<String>, sqlx::Error> {
    let pool = MySqlPool::connect(DB_URL).await?;
    let rows = sqlx::query("SELECT odds FROM match_odds_history ORDER BY id DESC LIMIT 10")
        .fetch_all(&pool)
        .await?;

    let mut odds_list = Vec::new();
    for row in rows {
        odds_list.push(row.try_get::<String, _>("odds")?);
    }
    Ok(odds_list)
}

/// 发送邮件
fn send_email(odds_data: &Vec<String>) {
    let email_body = format!("本周赔率数据:\n\n{:?}", odds_data);
    let email = Message::builder()
        .from(EMAIL_USER.parse().unwrap())
        .to(RECEIVER_EMAIL.parse().unwrap())
        .subject("每周赔率数据")
        .body(email_body)
        .unwrap();

    let creds = Credentials::new(EMAIL_USER.to_string(), EMAIL_PASS.to_string());
    let mailer = SmtpTransport::relay(SMTP_SERVER)
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => info!("邮件发送成功"),
        Err(err) => error!("邮件发送失败: {:?}", err),
    }
}

#[tokio::main]
async fn main() {
    // 获取赔率数据
    let odds_data = get_all_match_peilv();

    // 查询 MySQL 历史数据
    match query_mysql().await {
        Ok(history) => info!("查询历史数据: {:?}", history),
        Err(err) => error!("查询 MySQL 失败: {:?}", err),
    }

    // 发送邮件
    send_email(&odds_data);
}
