use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

fn main() {
    let total_winnings = number_of_steps(read_input_file());
    println!("the number of steps is {}", total_winnings);
}

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}

fn number_of_steps(input: String) -> u64 {
    let (steps, left_map, right_map, current_nodes) = parse_input(input);
    let (tx, rx): (Sender<u64>, Receiver<u64>) = mpsc::channel();
    let mut children = Vec::new();
    for node in current_nodes.clone() {
        let tx = tx.clone();
        let node = node.clone();
        let steps = steps.clone();
        let left_map = left_map.clone();
        let right_map = right_map.clone();
        let child = thread::spawn(move || {
            let mut node = node;
            let mut step_counter = 0;
            while !node.ends_with('Z') {
                for step in steps.chars() {
                    if step == 'L' {
                        node = left_map.get(&node).unwrap().to_string();
                    } else {
                        node = right_map.get(&node).unwrap().to_string();
                    }
                    step_counter += 1;
                }
            }
            tx.send(step_counter).unwrap();
        });
        children.push(child);
    }
    let mut start = true;
    let mut mcm = 1;
    for _ in current_nodes {
        let b = rx.recv().unwrap();
        if start {
            mcm = b;
            start = false;
        } else {
            mcm = calculate_mcm(std::cmp::max(mcm, b), std::cmp::min(mcm, b));
        }
    }
    mcm
}

fn calculate_mcm(max: u64, min: u64) -> u64 {
    let mut a = max;
    let mut b = min;
    while a % b != 0 {
        let tmp = a;
        a = b;
        b = tmp % a;
    }
    max * min / b
}

fn parse_input(
    input: String,
) -> (
    String,
    HashMap<String, String>,
    HashMap<String, String>,
    Vec<String>,
) {
    let mut steps = String::new();
    let mut left_map = HashMap::new();
    let mut right_map = HashMap::new();
    let mut parsing_node = false;
    let mut initial_nodes = Vec::new();
    for line in input.lines() {
        if !parsing_node {
            if line.trim().is_empty() {
                parsing_node = true;
            } else {
                steps = line.trim().to_string();
            }
        } else {
            let parts = line
                .split('=')
                .map(|s| s.trim())
                .map(|s| s.replace('(', ""))
                .map(|s| s.replace(')', ""))
                .filter(|s| !s.is_empty())
                .collect::<Vec<String>>();
            let neighbours = parts[1]
                .split(',')
                .map(|s| s.trim())
                .filter(|&s| !s.is_empty())
                .collect::<Vec<&str>>();
            left_map.insert(parts[0].clone(), neighbours[0].to_string());
            right_map.insert(parts[0].clone(), neighbours[1].to_string());
            if parts[0].ends_with('A') {
                initial_nodes.push(parts[0].clone());
            }
        }
    }
    (steps, left_map, right_map, initial_nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            .to_string();
        let result = number_of_steps(input);
        assert_eq!(result, 6);
    }
}
