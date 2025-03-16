// use mysql::*;
// use mysql::prelude::*;
// use reqwest::blocking::Client;
// use scraper::{Html, Selector};
// use serde_yaml::Value;
// use std::collections::HashMap;
// use std::fs::File;
// use std::io::Read;
// use std::sync::{Arc, Mutex};
// use std::thread;
// use std::time::Duration;
// use log::{info, error};
// use fern::Dispatch;

// // **读取 YAML 配置**
// fn read_yaml_config(file: &str) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
//     let mut file_content = String::new();
//     File::open(file)?.read_to_string(&mut file_content)?;
//     let yaml_data: HashMap<String, Value> = serde_yaml::from_str(&file_content)?;
//     Ok(yaml_data)
// }

// // **MySQL 连接池**
// fn get_mysql_pool(config: &HashMap<String, Value>) -> Result<PooledConn, Box<dyn std::error::Error>> {
//     let db_url = format!(
//         "mysql://{}:{}@{}/{}",
//         config["db"]["user"].as_str().unwrap(),
//         config["db"]["password"].as_str().unwrap(),
//         config["db"]["host"].as_str().unwrap(),
//         config["db"]["dbname"].as_str().unwrap()
//     );
//     let pool = Pool::new(db_url)?;
//     Ok(pool.get_conn()?)
// }

// // **初始化日志**
// fn init_logger() {
//     Dispatch::new()
//         .format(|out, message, record| {
//             out.finish(format_args!(
//                 "[{}] [{}] {}",
//                 chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
//                 record.level(),
//                 message
//             ));
//         })
//         .level(log::LevelFilter::Info)
//         .chain(std::io::stdout())
//         .chain(fern::log_file("log.log").unwrap())
//         .apply()
//         .unwrap();
// }

// // **获取比赛数据页面的 URL**
// fn get_match_urls(client: &Client) -> Vec<String> {
//     let url = "https://www.jleague.jp/match/";
//     let response = client.get(url).send().unwrap();

//     if response.status().is_success() {
//         let html = response.text().unwrap();
//         let document = Html::parse_document(&html);
//         let selector = Selector::parse("li a").unwrap();
        
//         let urls: Vec<String> = document
//             .select(&selector)
//             .filter_map(|el| el.value().attr("href"))
//             .filter(|&href| href.contains("live"))
//             .map(|href| format!("https://www.jleague.jp{}", href))
//             .collect();
        
//         info!("找到 {} 场比赛的 URL", urls.len());
//         urls
//     } else {
//         error!("无法访问 J-League 网站");
//         Vec::new()
//     }
// }

// // **爬取单场比赛数据**
// fn scrape_match_data(client: &Client, db_conn: Arc<Mutex<PooledConn>>, url: &str) {
//     let response = client.get(url).send().unwrap();
//     if response.status().is_success() {
//         let html = response.text().unwrap();
//         let document = Html::parse_document(&html);

//         let selector_team = Selector::parse("p span").unwrap();
//         let teams: Vec<String> = document
//             .select(&selector_team)
//             .map(|el| el.text().collect::<String>())
//             .collect();

//         if teams.len() >= 2 {
//             let home_team = &teams[0];
//             let away_team = &teams[teams.len() - 2];

//             let selector_score = Selector::parse(".leagLeftScore, .leagRightScore").unwrap();
//             let scores: Vec<String> = document
//                 .select(&selector_score)
//                 .map(|el| el.text().collect::<String>())
//                 .collect();

//             let home_score = scores.get(0).unwrap_or(&"0".to_string());
//             let away_score = scores.get(1).unwrap_or(&"0".to_string());

//             let insert_sql = format!(
//                 "INSERT INTO `j24` (date, home_team, away_team, home_score, away_score) VALUES (NOW(), '{}', '{}', {}, {})",
//                 home_team, away_team, home_score, away_score
//             );

//             let mut conn = db_conn.lock().unwrap();
//             if conn.query_drop(&insert_sql).is_ok() {
//                 info!("成功插入比赛数据: {} vs {} - {}:{}", home_team, away_team, home_score, away_score);
//             } else {
//                 error!("插入数据库失败: {}", insert_sql);
//             }
//         }
//     } else {
//         error!("无法获取比赛数据: {}", url);
//     }
// }

// // **主函数**
// fn main() {
//     init_logger();
//     let config = read_yaml_config("run.yml").expect("无法读取配置文件");
//     let db_conn = Arc::new(Mutex::new(get_mysql_pool(&config).expect("数据库连接失败")));

//     let client = Client::builder().danger_accept_invalid_certs(true).build().unwrap();
//     let urls = get_match_urls(&client);

//     let handles: Vec<_> = urls.iter().map(|url| {
//         let client = client.clone();
//         let db_conn = Arc::clone(&db_conn);
//         let url = url.clone();

//         thread::spawn(move || {
//             scrape_match_data(&client, db_conn, &url);
//             thread::sleep(Duration::from_secs(1));
//         })
//     }).collect();

//     for handle in handles {
//         handle.join().unwrap();
//     }
// }



// use headless_chrome::{Browser, LaunchOptions};
// use std::{thread, time::Duration};
// use lettre::{Message, SmtpTransport, Transport};
// use log::{info, error};


// fn main() {
//     // const URL: &str = "https://www.365-288.com/#/AC/B1/C1/D1002/E106068883/G938/H^1/";
//     const URL: &str = "https://www.baidu.com";
//     let browser = Browser::new(LaunchOptions::default()).expect("Failed to launch browser");
//     let tab = browser.new_tab().expect("Failed to open new tab");

//     // 打开网页
//     tab.navigate_to(URL).expect("Failed to navigate");
//     thread::sleep(Duration::from_secs(10)); // 等待页面加载
//     println!("allready load page");
//         // 执行 JavaScript 获取赔率数据
//         let js_script = r#"
//         let oddsData = [];
//         let elements = document.getElementsByClassName('src-ParticipantCenteredStacked48_Odds');
//         for (let i = 0; i < elements.length; i++) {
//             oddsData.push(elements[i].innerText);
//         }
//         oddsData;
//     "#;

//     match tab.evaluate(js_script, false) {
//         Ok(result) => {
//             if let Some(value) = result.get_value().as_array() {
//                 println!("value: {:?}", value);
//                 let odds: Vec<String> = value.iter().map(|v| v.as_str().map(String::from)).collect();
//                 return odds;
//             }
//             // if let Some(odds) = result.into_json::<Vec<String>>().ok() {
//             //     info!("获取赔率数据成功: {:?}", odds);
//             //     return odds;
//             // }
//         }
//         Err(err) => {
//             error!("获取赔率数据失败: {:?}", err);
//         }
//     }
//     vec![]
// }




use headless_chrome::{Browser, LaunchOptions};

const  URL: &str = "https://www.baidu.com";
fn main() {
    // 启动无头浏览器
    let browser = Browser::new(LaunchOptions::default()).expect("Failed to launch browser");

    // 创建一个新的标签页
    let tab = browser.new_tab().expect("Failed to open new tab");

    // 访问百度首页
    tab.navigate_to(URL)
        .expect("Failed to navigate to Baidu");

    // 等待搜索框元素加载
    tab.wait_for_element("#kw")
        .expect("Search input not found");

    // 获取搜索框的文本内容
    let search_box = tab.find_element("#su").expect("Failed to find search box");
    // let text = search_box.get_attribute("value").unwrap_or(Some("".to_string()));
    // let text = search_box.get_attribute_value("value").unwrap();
    let text = search_box.get_attribute_value("value").unwrap_or(Some("".to_string()));
    println!("百度搜索框中的文本: {:?}", text);

    // 关闭浏览器
    drop(browser);
}
