
use reqwest::{Client, Error};
use std::fs::File;
use std::io::{BufRead, BufReader};
use serde_json::json;

async fn get_all_esl() -> Result<Vec<String>, std::io::Error> {
    let file = File::open("eslid.txt")?;
    let reader = BufReader::new(file);

    let esl_list: Vec<String> = reader.lines()
        .filter_map(|line| line.ok())
        .collect();

    Ok(esl_list)
}

async fn get_and_delete_esl() -> Result<(), Error> {
    let esl_list = get_all_esl().await.expect("Failed to read ESL IDs");

    let client = Client::new();
    let login_data = json!({
        "username": "superuser",
        "password": "8b68d74d7f2ae790c501c55417c9b9bb"
    });

    let response = client.post("http://10.10.83.77:5288/prismart/weblogin")
        .json(&login_data)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Login successful!");
    } else {
        println!("Login failed! Status: {:?}", response.status());
        return Ok(());
    }

    for esl_id in esl_list {
        web_del_esl(&client, &esl_id).await?;
    }

    Ok(())
}

async fn web_del_esl(client: &Client, esl_id: &str) -> Result<(), Error> {
    let url = format!("http://10.10.83.77:5288/prismart/esl/wumart/001");
    let response = client.delete(&url)
        .json(&vec![esl_id])
        .send()
        .await?;

    if response.status().is_success() {
        println!("Successfully deleted ESL: {}", esl_id);
    } else {
        println!("Failed to delete ESL: {} | Status: {:?}", esl_id, response.status());
    }

    Ok(())
}


async fn run() {
    if let Err(e) = get_and_delete_esl().await {
        eprintln!("Error: {:?}", e);
    }
}
