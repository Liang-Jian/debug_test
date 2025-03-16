
use reqwest::{Client, Error};
use serde::{Serialize, Deserialize};
use serde_json::json;
use tokio;

const BASE_URL: &str = "http://172.17.120.25:9010";
const UC: &str = "shi.002";

#[derive(Debug, Serialize, Deserialize)]
struct EslRequest {
    esl_id: String,
    sid: String,
    priority: String,
    template: Option<String>,
    back_url: String,
    ap_mac: Option<String>,
    force_update: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ControlRequest {
    esl_id: String,
    sid: String,
    priority: String,
    back_url: String,
    switch_page: Option<SwitchPage>,
    flash_light: Option<FlashLight>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SwitchPage {
    page_id: String,
    stay_time: String,
    switch_page_mode: String,
    refresh_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FlashLight {
    colors: Vec<String>,
    on_time: String,
    off_time: String,
    sleep_time: String,
    loop_count: String,
    task_id: String,
}

struct EslApiClient {
    client: Client,
}

impl EslApiClient {
    fn new() -> Self {
        EslApiClient {
            client: Client::new(),
        }
    }

    async fn update(&self, esl_list: &[&str]) -> Result<(), Error> {
        let params: Vec<_> = esl_list.iter().map(|&e| EslRequest {
            esl_id: e.to_string(),
            sid: "19940510".to_string(),
            priority: "1".to_string(),
            template: Some("sct".to_string()),
            back_url: "http://10.11.173.32:9091/shopweb-webapp/ogi/ew/httpHandler".to_string(),
            ap_mac: Some("98:6D:35:76:6D:B8".to_string()),
            force_update: Some("false".to_string()),
        }).collect();

        let body = json!({ "data": params });

        let res = self.client.put(format!("{}/api3/default/esls", BASE_URL))
            .json(&body)
            .send()
            .await?;

        println!("Update finished: {:?}", res.text().await?);
        Ok(())
    }

    async fn control(&self, esl_list: &[&str]) -> Result<(), Error> {
        let params: Vec<_> = esl_list.iter().map(|&e| ControlRequest {
            esl_id: e.to_string(),
            sid: "19940510".to_string(),
            priority: "1".to_string(),
            back_url: "http://10.11.173.32:9091/shopweb-webapp/ogi/ew/httpHandler".to_string(),
            switch_page: Some(SwitchPage {
                page_id: "".to_string(),
                stay_time: "".to_string(),
                switch_page_mode: "".to_string(),
                refresh_type: "".to_string(),
            }),
            flash_light: Some(FlashLight {
                colors: vec!["blue".to_string(), "white".to_string()],
                on_time: "".to_string(),
                off_time: "".to_string(),
                sleep_time: "".to_string(),
                loop_count: "".to_string(),
                task_id: "".to_string(),
            }),
        }).collect();

        let body = json!({ "data": params });

        let res = self.client.put(format!("{}/api3/default/control", BASE_URL))
            .json(&body)
            .send()
            .await?;

        println!("Control finished: {:?}", res.text().await?);
        Ok(())
    }

    async fn query_esl_info(&self, esl_id: &str) -> Result<(), Error> {
        let body = json!({
            "data": [{
                "esl_id": esl_id,
                "sid": "8964123",
                "type": 51,
                "back_url": "http://10.11.163.211:8080/shopweb-webapp/ogi/ew/httpHandler"
            }]
        });

        let res = self.client.put(format!("{}/api3/{}/esls/query/statistics", BASE_URL, UC))
            .json(&body)
            .send()
            .await?;

        println!("Query finished: {:?}", res.text().await?);
        Ok(())
    }
}


async fn run() -> Result<(), Error> {
    let esl_client = EslApiClient::new();
    let esl_list = ["61-8B-37-88"];

    esl_client.update(&esl_list).await?;
    esl_client.control(&esl_list).await?;
    esl_client.query_esl_info(esl_list[0]).await?;

    Ok(())
}
