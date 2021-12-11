use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let mut lantern_fishes = vec![];
    for line in contents.lines() {
        lantern_fishes = line.split(",").map(|x| x.trim().parse::<i32>().unwrap()).collect();
    }
    let time = 80;
    println!("Initial state: {:?}", lantern_fishes);
    for i in 1..time + 1 {
        let mut new_fishes = vec![];
        for fish in lantern_fishes.clone() {
            if fish == 0 {
                new_fishes.push(6);
                new_fishes.push(8);
            } else {
                new_fishes.push(fish - 1);
            }
        }
        lantern_fishes = new_fishes;
        println!("After {} days: {:?}", i, lantern_fishes);
    }
    println!("Total fish: {}", lantern_fishes.len());
}
