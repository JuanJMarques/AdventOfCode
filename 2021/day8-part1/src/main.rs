use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");
    let mut unique_digits_counter = 0;
    for line in contents.lines() {
        let output = line.split("|").last().expect("count not find output");
        unique_digits_counter += output.split(" ").into_iter()
                .map(|x| {
                    let mut unique_digit_value = 0;
                    if is_unique_segment_digits(count_segments(x.trim())) {
                        unique_digit_value = 1
                    }
                    unique_digit_value
                })
                .sum::<u32>();
    }
    println!("number of unique digits {}", unique_digits_counter);
}

fn is_unique_segment_digits(segments_number: u32) -> bool {
    match segments_number {
        2 | 3 | 4 | 7 => true,
        _ => false,
    }
}

fn count_segments(segments: &str) -> u32 {
    segments.len() as u32
}