



use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct ESL {
    esl_id: String,
    task_id: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ESLStats {
    esl_id: String,
    ack_value: String,
    task_id: String,
    retry_time: i32,
    success_num: i32,
    fail_num: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskData {
    task_id: String,
    ack_data: Vec<ESLStats>,
}
