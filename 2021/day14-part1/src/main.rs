use std::collections::HashMap;
use std::fs::{File};
use std::io::{Read};


fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");
    println!("Most common element frequency minus least common element frequency: {}", emulate_cycles(contents, 10));
}

fn emulate_cycles(contents: String, num_cycles: i32) -> u32 {
    let (insertion_rules, mut polymer_template) = parse_input(contents);
    for _ in 0..num_cycles {
        polymer_template = apply_rules(polymer_template, &insertion_rules);
    }
    let frequency_map = get_frequency_map(polymer_template);
    (frequency_map.values().map(|x| *x as u32).max().unwrap() - frequency_map.values().map(|x| *x as u32).min().unwrap()) as u32
}

fn get_frequency_map(polymer: Vec<char>) -> HashMap<char, i32> {
    let mut frequency_map = HashMap::new();
    for c in polymer {
        let count = frequency_map.entry(c).or_insert(0);
        *count += 1;
    }
    frequency_map
}

fn apply_rules(polymer_template: Vec<char>, rules: &HashMap<(char, char), char>) -> Vec<char> {
    let mut previous_char = Option::None;
    let mut new_polymer = Vec::new();
    for current_char in polymer_template {
        if let Some(previous_char) = previous_char {
            new_polymer.push(previous_char);
            if let Some(replacement_char) = rules.get(&(previous_char, current_char)) {
                new_polymer.push(*replacement_char);
            }
        }
        previous_char = Some(current_char);
    }
    new_polymer.push(previous_char.unwrap());
    new_polymer
}

fn parse_input(table_str: String) -> (HashMap<(char, char), char>, Vec<char>) {
    let mut insertion_rules = HashMap::new();
    let mut polymer_template = vec![];
    let mut parse_insertions = false;
    for line in table_str.lines() {
        if line.trim().is_empty() {
            parse_insertions = true;
            continue;
        }
        if parse_insertions {
            let mut split = line.split("->");
            let mut left = split.next().unwrap().trim().chars();
            let mut right = split.next().unwrap().trim().chars();
            let combination = (left.next().unwrap(), left.next().unwrap());
            let result = right.next().unwrap();
            insertion_rules.insert(combination, result);
        } else {
            polymer_template = line.chars().collect::<Vec<char>>();
        }
    }
    (insertion_rules, polymer_template)
}

#[test]
fn test_case_day14_1() {
    let points = emulate_cycles(
        String::from("NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"), 10);
    assert_eq!(points, 1588)
}

