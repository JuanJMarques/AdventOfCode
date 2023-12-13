use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

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

fn number_of_steps(input: String) -> u32 {
    let (steps, left_map, right_map) = parse_input(input);
    let mut step_counter = 0;
    let mut current_node = "AAA".to_string();
    while current_node != *"ZZZ" {
        for step in steps.chars() {
            if step == 'L' {
                current_node = left_map.get(&current_node).unwrap().to_string();
            } else {
                current_node = right_map.get(&current_node).unwrap().to_string();
            }
            step_counter += 1
        }
    }
    step_counter
}

fn parse_input(input: String) -> (String, HashMap<String, String>, HashMap<String, String>) {
    let mut steps = String::new();
    let mut left_map = HashMap::new();
    let mut right_map = HashMap::new();
    let mut parsing_node = false;
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
            let neightbourgs = parts[1]
                .split(',')
                .map(|s| s.trim())
                .filter(|&s| !s.is_empty())
                .collect::<Vec<&str>>();
            left_map.insert(parts[0].clone(), neightbourgs[0].to_string());
            right_map.insert(parts[0].clone(), neightbourgs[1].to_string());
        }
    }
    (steps, left_map, right_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            .to_string();
        let result = number_of_steps(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_input_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            .to_string();
        let result = number_of_steps(input);
        assert_eq!(result, 6);
    }
}
