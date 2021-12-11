use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");
    let mut lantern_fishes = HashMap::new();
    for line in contents.lines() {
        lantern_fishes = create_fishes_map(line.split(",").map(|x| x.trim().parse::<i32>().unwrap()).collect());
    }
    let time = 256;
    println!("Initial state: {:?}", lantern_fishes);
    for i in 1..time + 1 {
        let mut new_fishes: u64 = 0;
        for j in 0..9 {
            if j == 0 {
                new_fishes = *lantern_fishes.get(&j).expect(format!("No fish at {}", j).as_str());
            } else {
                let fishes = *lantern_fishes.get(&j).expect(format!("No fish at {}", j).as_str());
                lantern_fishes.insert(j - 1, fishes);
                lantern_fishes.insert(j, 0);
            }
        }
        lantern_fishes.insert(8, new_fishes);
        let fishes_with_6 = lantern_fishes.get(&6).expect(format!("No fish at {}", 6).as_str());
        lantern_fishes.insert(6, new_fishes + fishes_with_6);
        println!("after {} days: {:?}", i, lantern_fishes);
    }
    println!("Total fishes: {}", lantern_fishes.values().sum::<u64>());
}

fn create_fishes_map(fishes: Vec<i32>) -> HashMap<i32, u64> {
    let mut fishes_map = HashMap::new();
    for i in 0..9 {
        fishes_map.insert(i, 0);
    }
    for fish in fishes {
        let count = fishes_map.get(&fish).unwrap();
        fishes_map.insert(fish, count + 1);
    }
    fishes_map
}
