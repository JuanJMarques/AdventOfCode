use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");
    let mut crab_positions = vec![];
    for line in contents.lines() {
        crab_positions = line.split(",").map(|x| x.trim().parse::<i32>().unwrap()).collect();
    }
    let mut min_fuel_cost = std::i32::MAX;
    for test_position in crab_positions.clone() {
        let mut current_fuel_cost = 0;
        for crab_position in crab_positions.clone() {
            current_fuel_cost += (test_position - crab_position).abs();
        }
        min_fuel_cost = std::cmp::min(min_fuel_cost, current_fuel_cost);
    }
    println!("min fuel cost: {}", min_fuel_cost);
}