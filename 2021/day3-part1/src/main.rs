use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let mut first_line = true;
    let mut count_zeroes: Vec<u32> = Vec::new();
    let mut count_ones: Vec<u32> = Vec::new();
    for mut line in contents.lines() {
        line = line.trim();
        if first_line {
            first_line = false;
            count_zeroes = vec![0; line.len()];
            count_ones = vec![0; line.len()];
        }
        for i in 0..line.chars().count() {
            match line.chars().nth(i).unwrap() {
                '0' => count_zeroes[i] += 1,
                '1' => count_ones[i] += 1,
                _ => panic!("Invalid character"),
            }
        }
    }
    println!("count zeroes {:?}", count_zeroes);
    println!("count ones {:?}", count_ones);
    let most_common_bit = count_ones.iter().zip(count_zeroes.iter()).map(|(a, b)| {
        if a > b {
            1
        } else {
            0
        }
    }).collect::<Vec<u32>>();
    let least_common_bit = count_ones.iter().zip(count_zeroes.iter()).map(|(a, b)| {
        if a < b {
            1
        } else {
            0
        }
    }).collect::<Vec<u32>>();
    println!("most common bit {:?}", most_common_bit);
    println!("least common bit {:?}", least_common_bit);
    let gamma = most_common_bit.iter().rev().fold((0, 1), |(acc, mul), &bit| (acc + (mul * (1 & bit as u32)), mul.wrapping_add(mul))).0;
    let epsilon = least_common_bit.iter().rev().fold((0, 1), |(acc, mul), &bit| (acc + (mul * (1 & bit as u32)), mul.wrapping_add(mul))).0;
    println!("gamma {}", gamma);
    println!("epsilon {}", epsilon);
    println!("power consumption {}", gamma * epsilon);
}
