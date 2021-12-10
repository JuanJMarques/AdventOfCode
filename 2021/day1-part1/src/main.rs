use std::fs::File;
use std::io::Read;

fn main() {
    let mut previous_measure: Option<i32> = None;
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    let mut increasing_measures = 0;
    file.read_to_string(&mut contents);
    for line in contents.lines() {
        let actual_measure = line.parse::<i32>().unwrap();
        if previous_measure.is_some() && actual_measure > previous_measure.unwrap() {
            println!("{} (Increasing)", actual_measure);
            increasing_measures += 1;
        } else if previous_measure.is_none() {
            println!("{} (N/A - no previous measurement)", actual_measure);
        } else if previous_measure.unwrap() == actual_measure {
            println!("{} (Same)", actual_measure);
        } else {
            println!("{} (Decreasing)", actual_measure);
        }

        previous_measure = Some(actual_measure);
    }
    println!("Number of increasing measures: {}", increasing_measures);
}