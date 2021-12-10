use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").expect("File not Found");
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let mut history: Vec<Vec<u32>> = Vec::new();
    for mut line in contents.lines() {
        history.push(line
                .trim()
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>());
    }
    let mut oxygen_history = history.clone();
    let mut pos: usize = 0;
    while oxygen_history.len() > 1 {
        println!("filtering oxygen history: {:?}", oxygen_history);
        let mut count_zeroes: u32 = 0;
        let mut count_ones: u32 = 0;
        oxygen_history.iter().for_each(|x| {
            match x[pos] {
                0 => count_zeroes += 1,
                1 => count_ones += 1,
                _ => panic!("Invalid value"),
            }
        });
        let oxygen_history_copy = oxygen_history.clone();
        oxygen_history.clear();
        if count_ones >= count_zeroes {
            oxygen_history_copy
                    .iter()
                    .filter(|x| x[pos] == 1)
                    .for_each(|x| oxygen_history.push(x.clone()));
        } else {
            oxygen_history_copy
                    .iter()
                    .filter(|x| x[pos] == 0)
                    .for_each(|x| oxygen_history.push(x.clone()));
        }
        pos += 1;
    }
    pos = 0;
    let mut co2_history = history.clone();
    while co2_history.len() > 1 {
        println!("filtering co2 history: {:?}", co2_history);
        let mut count_zeroes: u32 = 0;
        let mut count_ones: u32 = 0;
        co2_history.iter().for_each(|x| {
            match x[pos] {
                0 => count_zeroes += 1,
                1 => count_ones += 1,
                _ => panic!("Invalid value"),
            }
        });
        let co2_history_copy = co2_history.clone();
        co2_history.clear();
        if count_ones < count_zeroes {
            co2_history_copy
                    .iter()
                    .filter(|x| x[pos] == 1)
                    .for_each(|x| co2_history.push(x.clone()));
        } else {
            co2_history_copy
                    .iter()
                    .filter(|x| x[pos] == 0)
                    .for_each(|x| co2_history.push(x.clone()));
        }
        pos += 1;
    }
    println!("filtered oxygen history {:?}", oxygen_history);
    if oxygen_history.len() > 1 {
        panic!("oxygen history should have only 1 element after filtering");
    }
    let oxigen_rating = oxygen_history[0].iter().rev().fold((0, 1), |(acc, mul), &bit| (acc + (mul * (1 & bit as u32)), mul.wrapping_add(mul))).0;
    println!("filtered co2 history {:?}", co2_history);
    if co2_history.len() > 1 {
        panic!("co2 history should have only 1 element after filtering");
    }
    let co2_rating = co2_history[0].iter().rev().fold((0, 1), |(acc, mul), &bit| (acc + (mul * (1 & bit as u32)), mul.wrapping_add(mul))).0;
    println!("oxygen rating {}, co2 rating {}, life plan support rating {}", oxigen_rating, co2_rating, oxigen_rating * co2_rating);
}
