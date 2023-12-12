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
    let mut times: Vec<u64> = vec![];
    let mut distances: Vec<u64> = vec![];
    let mut total = 1;
    for line in input.lines() {
        if line.contains("Time") {
            times.push(
                line.split(' ')
                    .map(|s| s.trim())
                    .filter(|&s| !(s.is_empty() || s.contains("Time") || s.contains("Distance")))
                    .fold(String::new(), |acc, s| {
                        (acc.to_owned() + s.to_owned().as_str())
                    })
                    .parse::<u64>()
                    .unwrap(),
            );
        } else {
            distances.push(
                line.split(' ')
                    .map(|s| s.trim())
                    .filter(|&s| !(s.is_empty() || s.contains("Time") || s.contains("Distance")))
                    .fold(String::new(), |acc, s| {
                        (acc.to_owned() + s.to_owned().as_str())
                    })
                    .parse::<u64>()
                    .unwrap(),
            );
        }
    }
    for (i, &distance) in distances.iter().enumerate() {
        total *= calculate_number_of_ways(times[i], distance);
    }
    total
}

fn calculate_number_of_ways(total_time: u64, distance: u64) -> u64 {
    let mut number_of_ways = 0;
    for i in 1..total_time {
        if beats_record_distance(i, total_time, distance) {
            number_of_ways += 1
        }
    }
    number_of_ways
}

fn beats_record_distance(pressed_time: u64, total_time: u64, distance: u64) -> bool {
    distance < calculate_distance(pressed_time, total_time)
}

fn calculate_distance(pressed_time: u64, total_time: u64) -> u64 {
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
        assert_eq!(result, 71503);
    }
}
