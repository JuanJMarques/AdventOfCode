use std::fs::File;
use std::io::Read;

fn main() {
    let multiplied_number_of_ways = multiply_number_of_ways(read_input_file());
    println!(
        "the number of ways to beat the record multiplied is {}",
        multiplied_number_of_ways
    );
}

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}

fn multiply_number_of_ways(input: String) -> u64 {
    let mut times: Vec<u32> = vec![];
    let mut distances: Vec<u32> = vec![];
    let mut total = 1;
    for line in input.lines() {
        if line.contains("Time") {
            times = line
                .split(' ')
                .map(|s| s.trim())
                .filter(|&s| !(s.is_empty() || s.contains("Time") || s.contains("Distance")))
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
        } else {
            distances = line
                .split(' ')
                .map(|s| s.trim())
                .filter(|&s| !(s.is_empty() || s.contains("Time") || s.contains("Distance")))
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
        }
    }
    for (i, &distance) in distances.iter().enumerate() {
        total *= calculate_number_of_ways(times[i], distance) as u64;
    }
    total
}

fn calculate_number_of_ways(total_time: u32, distance: u32) -> u32 {
    let mut number_of_ways = 0;
    for i in 1..total_time {
        if beats_record_distance(i, total_time, distance) {
            number_of_ways += 1
        }
    }
    number_of_ways
}

fn beats_record_distance(pressed_time: u32, total_time: u32, distance: u32) -> bool {
    distance < calculate_distance(pressed_time, total_time)
}

fn calculate_distance(pressed_time: u32, total_time: u32) -> u32 {
    (pressed_time * total_time) - (pressed_time * pressed_time)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = "Time:      7  15   30
Distance:  9  40  200"
            .to_string();
        let result = multiply_number_of_ways(input);
        assert_eq!(result, 288);
    }
}
