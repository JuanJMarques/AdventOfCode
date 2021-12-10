use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    let mut increasing_measures = 0;
    file.read_to_string(&mut contents);
    let mut sliding_windows = vec![];
    let mut line_count = 0;
    for line in contents.lines() {
        let actual_measure = line.parse::<i32>().unwrap();
        sliding_windows.push(Vec::new());
        for i in 0..3 {
            if line_count >= i {
                sliding_windows.get_mut(line_count - i).unwrap().push(actual_measure);
            }
        }
        line_count += 1;
    }
    let mut sum_windows: Vec<i32> = vec![];
    for i in 0..sliding_windows.len() {
        if sliding_windows.get(i).unwrap().len() == 3 {
            sum_windows.push(sliding_windows.get(i).unwrap().iter().sum());
        }
    }
    let mut previous_measure: Option<i32> = None;
    for i in 0..sum_windows.len() {
        let actual_measure = *sum_windows.get(i).unwrap();
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