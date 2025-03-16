


use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn read_white_list(white_list_path: &str) -> HashSet<String> {
    let file = File::open(white_list_path).expect("Failed to open white list file");
    let reader = BufReader::new(file);
    reader.lines()
        .map(|line| line.unwrap().trim().to_string())
        .collect()
}

fn location_by_anchor_counter(log_path: &str, white_list_path: &str) {
    let white_list = read_white_list(white_list_path);
    let log_file = File::open(log_path).expect("Failed to open log file");
    let reader = BufReader::new(log_file);

    let mut esl_id_is_calculated: HashMap<String, Vec<String>> = HashMap::new();
    let node_regex = Regex::new(r"DEBUG - nodeId = (.+?), paths = \[(.+?)\]").unwrap();
    
    for line in reader.lines() {
        let line = line.unwrap();
        if let Some(captures) = node_regex.captures(&line) {
            let node_id = captures[1].to_string();
            let paths = captures[2].to_string();
            if white_list.contains(&node_id) {
                esl_id_is_calculated.entry(node_id).or_default().push(paths);
            }
        }
    }

    let mut neighbor_count = 0;
    let mut anchor_count = 0;
    let path_regex = Regex::new(r"Path\{nodeId='(.+?)', weightValue=(.+?), anchorPath=(.+?), sign=(.+?)\}").unwrap();

    for (_key, value) in &esl_id_is_calculated {
        if let Some(last_path) = value.last() {
            for cap in path_regex.captures_iter(last_path) {
                neighbor_count += 1;
                if &cap[4] == "1" {
                    anchor_count += 1;
                }
            }
        }
    }

    println!("Total ESL IDs calculated: {}, Neighbors count: {}, Location by anchor: {}",
        esl_id_is_calculated.len(), neighbor_count, anchor_count);
}

fn not_received_set_command_error(geo_path: &str, white_list_path: &str) {
    let white_list = read_white_list(white_list_path);
    let geo_file = File::open(geo_path).expect("Failed to open geolocation file");
    let reader = BufReader::new(geo_file);

    let mut esl_id_set1: HashSet<String> = HashSet::new();
    let mut esl_id_set3: HashSet<String> = HashSet::new();

    let neighbor_regex = Regex::new(r"DEBUG - \{(.+?)\}.*nodeId == (.+?), neighbors == (.+?)").unwrap();
    let power_regex = Regex::new(r"DEBUG - \{(.+?)\}.*nodeId == (.+?), neighborsPower == \{(.+?)\}").unwrap();

    for line in reader.lines() {
        let line = line.unwrap();
        if let Some(cap) = neighbor_regex.captures(&line) {
            let node_id = cap[2].to_string();
            if white_list.contains(&node_id) {
                esl_id_set1.insert(node_id);
            }
        }
        if let Some(cap) = power_regex.captures(&line) {
            let neighbors: Vec<&str> = cap[3].split(", ").collect();
            for neighbor in neighbors {
                if let Some((id, _)) = neighbor.split_once("=") {
                    if white_list.contains(id) {
                        esl_id_set3.insert(id.to_string());
                    }
                }
            }
        }
    }

    let all_id: HashSet<_> = esl_id_set1.union(&esl_id_set3).collect();
    let esl_id_inexistence: HashSet<_> = white_list.difference(&all_id).collect();

    println!("Not Received Set Command Counter: {}", esl_id_inexistence.len());

    let mut file = File::create("noset.txt").expect("Failed to create file");
    for id in esl_id_inexistence {
        writeln!(file, "{}", id).expect("Failed to write to file");
    }
}

fn packet_lost(log_path: &str, white_list_path: &str) {
    let white_list = read_white_list(white_list_path);
    let log_file = File::open(log_path).expect("Failed to open log file");
    let reader = BufReader::new(log_file);

    let mut src: HashSet<(String, String, String, String, String)> = HashSet::new();
    let packet_regex = Regex::new(r"DEBUG - \{(.+?)\}.*nodeId == (.+?), esl count = (.+?), package count = (.+?), package num = (.+?)").unwrap();

    for line in reader.lines() {
        let line = line.unwrap();
        if let Some(cap) = packet_regex.captures(&line) {
            let node_id = cap[2].to_string();
            if white_list.contains(&node_id) {
                src.insert((
                    cap[1].to_string(),
                    cap[2].to_string(),
                    cap[3].to_string(),
                    cap[4].to_string(),
                    cap[5].to_string(),
                ));
            }
        }
    }

    let mut esl_id_has_no_neighbors = Vec::new();
    let mut esl_id_lost_packet = Vec::new();
    let mut total_packets: usize = 0;
    let mut received_packets: usize = 0;

    for (log_time, node_id, esl_count, package_count, package_num) in &src {
        if esl_count == "0" {
            esl_id_has_no_neighbors.push(node_id.clone());
        }
        total_packets += package_num.parse::<usize>().unwrap_or(0);
        received_packets += 1;
    }

    let packet_lost_rate = 1.0 - (received_packets as f64 / total_packets as f64);
    
    println!("Packet Loss Rate: {:.2}%", packet_lost_rate * 100.0);
    println!("Number of ESLs without neighbors: {}", esl_id_has_no_neighbors.len());

    let mut file = File::create("NoNeighbors.txt").expect("Failed to create file");
    for id in esl_id_has_no_neighbors {
        writeln!(file, "{}", id).expect("Failed to write to file");
    }
}

fn main() {
    let log_path = "geolocation.log";
    let white_list_path = "id.txt";

    location_by_anchor_counter(log_path, white_list_path);
    not_received_set_command_error(log_path, white_list_path);
    packet_lost(log_path, white_list_path);
}
