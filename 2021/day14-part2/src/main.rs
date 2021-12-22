mod threadpool;

use std::collections::HashMap;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Write};
use std::path::Path;
use std::sync::{Arc, mpsc};
use crate::threadpool::ThreadPool;


fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");
    println!("Most common element frequency minus least common element frequency: {}", emulate_cycles(contents, 40));
}

fn emulate_cycles(contents: String, num_cycles: i32) -> u64 {
    let (insertion_rules, mut polymer_template) = parse_input(contents);
    let thread_pool: ThreadPool = ThreadPool::new(10);
    let memory_file = "mem.txt";
    let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(memory_file)
            .expect("Failed to open output file");
    file.write_all(&*polymer_template.iter().map(|c| *c as u8).collect::<Vec<u8>>()).expect("Failed to write to output file");
    drop(file);
    for x in 0..num_cycles {
        println!("Cycle {}", x);
        // polymer_template = apply_rules(polymer_template, &insertion_rules, Option::Some(&thread_pool));
        // println!("{}", polymer_template.len());
        apply_rules_file(memory_file, &insertion_rules);
    }
    let frequency_map = get_frequency_map_file(memory_file);
    (frequency_map.values().map(|x| *x as u64).max().unwrap() - frequency_map.values().map(|x| *x as u64).min().unwrap()) as u64
}

fn apply_rules_file(file_name: &str, rules: &HashMap<(char, char), char>) {
    let mut input_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(file_name)
            .expect("Failed to open input file");
    let temp_output = "output.txt";
    let mut output_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(temp_output)
            .expect("Failed to open output file");
    let mut buffer = vec![0; 1024 * 1024];
    let mut previous = Option::None;
    while let Ok(size) = input_file.read(&mut buffer) {
        if size == 0 {
            break;
        }
        let mut output_buffer = vec![];
        let chars = buffer.iter().take(size)
                .map(|x| *x as char)
                .collect::<Vec<char>>();
        if let Some(prev_char) = previous {
            if let Some(replacement_char) = rules.get(&(prev_char, chars[0])) {
                output_buffer.push(*replacement_char);
            }
        }
        output_buffer.push(chars[0]);
        for window in chars.windows(2) {
            let prev = window[0];
            let next = window[1];
            if let Some(replacement_char) = rules.get(&(prev, next)) {
                output_buffer.push(*replacement_char)
            }
            output_buffer.push(next);
        }
        output_buffer.push(chars[chars.len() - 1]);
        previous = Some(chars[chars.len() - 1]);
        let mut output_bytes = output_buffer.iter().map(|c| *c as u8).collect::<Vec<u8>>();
        output_file.write_all(&output_bytes).expect("Failed to write to output file");
    }
    drop(input_file);
    drop(output_file);
    fs::remove_file(Path::new(file_name)).expect("Failed to remove input file");
    fs::rename(Path::new(temp_output), Path::new(file_name)).expect("Failed to rename output file");
}

fn get_frequency_map_file(file_name: &str) -> HashMap<char, i64> {
    let mut frequency_map = HashMap::new();
    let mut input_file = OpenOptions::new()
            .read(true)
            .open(file_name)
            .expect("Failed to open input file");
    let mut buffer = vec![0; 1024 * 1024 * 1024];
    while let Ok(size) = input_file.read(&mut buffer) {
        if size == 0 {
            break;
        }
        let chars = buffer.iter().take(size)
                .map(|x| *x as char)
                .collect::<Vec<char>>();
        for c in chars {
            let count = frequency_map.entry(c).or_insert(0);
            *count += 1;
        }
    }
    frequency_map
}

fn get_frequency_map(polymer: Vec<char>) -> HashMap<char, i64> {
    let mut frequency_map = HashMap::new();
    for c in polymer {
        let count = frequency_map.entry(c).or_insert(0);
        *count += 1;
    }
    frequency_map
}

fn apply_rules(polymer_template: Vec<char>, rules: &HashMap<(char, char), char>, thread_pool_opt: Option<&ThreadPool>) -> Vec<char> {
    let mut new_polymer = Vec::new();
    // const SIZE: usize = 15000;
    // if polymer_template.len() >= SIZE && thread_pool_opt.is_some() {
    //     let thread_pool = thread_pool_opt.unwrap();
    //     let mut results = Vec::new();
    //     let rules_arc = Arc::new(rules.clone());
    //     for i in 0..polymer_template.len() / SIZE {
    //         let (start, end) = (i * SIZE, ((i + 1) * SIZE).min(polymer_template.len()));
    //         let polymer_template_slice = polymer_template[start..end].to_vec();
    //         let rules_arc_clone = Arc::clone(&rules_arc);
    //         let (tx, rx) = mpsc::channel();
    //         thread_pool.execute(move || {
    //             tx.send(apply_rules(polymer_template_slice, &*rules_arc_clone, Option::None)).unwrap();
    //         });
    //         results.push(rx);
    //         if (i + 1) * SIZE < polymer_template.len() {
    //             let polymer_template_slice = polymer_template[(i + 1) * SIZE - 1..(i + 1) * SIZE].to_vec();
    //             let rules_arc_clone = Arc::clone(&rules_arc);
    //             let (tx, rx) = mpsc::channel();
    //             thread_pool.execute(move || {
    //                 tx.send(apply_rules(polymer_template_slice, &*rules_arc_clone, Option::None)).unwrap();
    //             });
    //             results.push(rx);
    //         }
    //     }
    //     for rx in results {
    //         let result = rx.recv().unwrap();
    //         new_polymer.extend(result);
    //     }
    // } else {
    new_polymer.push(polymer_template[0]);
    for window in polymer_template.windows(2) {
        let prev = window[0];
        let next = window[1];
        if let Some(replacement_char) = rules.get(&(prev, next)) {
            new_polymer.push(*replacement_char)
        }
        new_polymer.push(next);
    }
    // }
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
fn test_case_day14_2() {
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
CN -> C"), 40);
    assert_eq!(points, 2188189693529)
}

