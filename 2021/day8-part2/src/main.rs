use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");
    let mut numbers_sum = 0;
    for line in contents.lines() {
        let mut digits_map = HashMap::new();
        let mut segments_map = HashMap::new();
        for i in 0..10 {
            digits_map.insert(i, Vec::new());
        }
        let mut split = line.split("|");
        let input = split.next().into_iter().map(parse_digits)
                .fold(Vec::new(), |mut acc, mut x| {
                    acc.append(&mut x);
                    acc
                });
        let mut output = split.next().into_iter().map(parse_digits)
                .fold(Vec::new(), |mut acc, mut x| {
                    acc.append(&mut x);
                    acc
                });
        let mut all_digits = input.clone();
        all_digits.append(&mut output.clone());
        find_unique_digits(all_digits.clone(), &mut digits_map);
        find_upper_segment(digits_map.clone(), &mut segments_map);
        find_nine_digit(all_digits.clone(), &mut digits_map);
        find_lower_segment(digits_map.clone(), &mut segments_map);
        find_three_digit(all_digits.clone(), &mut digits_map);
        find_upper_left_segment(digits_map.clone(), &mut segments_map);
        find_five_digit(all_digits.clone(), segments_map.clone(), &mut digits_map);
        find_upper_right_segment(digits_map.clone(), &mut segments_map);
        find_lower_right_segment(digits_map.clone(), &mut segments_map);
        find_middle_segment(digits_map.clone(), &mut segments_map);
        find_six_digit(all_digits.clone(), segments_map.clone(), &mut digits_map);
        find_lower_left_segment(digits_map.clone(), &mut segments_map);
        println!("segments map: {:?}", segments_map);
        let reverse_segment_map = inverse_segments_map(segments_map.clone());
        let number = parse_output_number(output, reverse_segment_map.clone());
        println!("parsed output number: {}", number);
        numbers_sum += number;
    }
    println!("output sum: {}", numbers_sum);
}

fn inverse_segments_map(segments_map: HashMap<u8, char>) -> HashMap<char, u8> {
    let mut inverse_segments_map = HashMap::new();
    for (key, value) in segments_map {
        inverse_segments_map.insert(value, key);
    }
    inverse_segments_map
}

fn parse_output_number(digits: Vec<&str>, segments_map: HashMap<char, u8>) -> u32 {
    let mut number: u32 = 0;
    for digit in digits {
        number = number * 10;
        let mut segments = vec![];
        for c in digit.chars() {
            let mut map = segments_map.clone();
            let option = map.get_mut(&c);
            match option {
                Some(segment) => {
                    segments.push(*segment);
                }
                None => {
                    panic!("No segment found for {}", c);
                }
            }
        }
        let mut digit_number = parse_digit_by_segments(segments);
        number += digit_number;
    }
    number
}

fn parse_digit_by_segments(segments: Vec<u8>) -> u32 {
    if segments.contains(&(0 as u8))
            && segments.contains(&(1 as u8))
            && segments.contains(&(2 as u8))
            && segments.contains(&(3 as u8))
            && segments.contains(&(4 as u8))
            && segments.contains(&(5 as u8))
            && !segments.contains(&(6 as u8)) {
        return 0;
    } else if !segments.contains(&(0 as u8))
            && segments.contains(&(1 as u8))
            && segments.contains(&(2 as u8))
            && !segments.contains(&(3 as u8))
            && !segments.contains(&(4 as u8))
            && !segments.contains(&(5 as u8))
            && !segments.contains(&(6 as u8)) {
        return 1;
    } else if segments.contains(&(0 as u8))
            && segments.contains(&(1 as u8))
            && !segments.contains(&(2 as u8))
            && segments.contains(&(3 as u8))
            && segments.contains(&(4 as u8))
            && !segments.contains(&(5 as u8))
            && segments.contains(&(6 as u8)) {
        return 2;
    } else if segments.contains(&(0 as u8))
            && segments.contains(&(1 as u8))
            && segments.contains(&(2 as u8))
            && segments.contains(&(3 as u8))
            && !segments.contains(&(4 as u8))
            && !segments.contains(&(5 as u8))
            && segments.contains(&(6 as u8)) {
        return 3;
    } else if !segments.contains(&(0 as u8))
            && segments.contains(&(1 as u8))
            && segments.contains(&(2 as u8))
            && !segments.contains(&(3 as u8))
            && !segments.contains(&(4 as u8))
            && segments.contains(&(5 as u8))
            && segments.contains(&(6 as u8)) {
        return 4;
    } else if segments.contains(&(0 as u8))
            && !segments.contains(&(1 as u8))
            && segments.contains(&(2 as u8))
            && segments.contains(&(3 as u8))
            && !segments.contains(&(4 as u8))
            && segments.contains(&(5 as u8))
            && segments.contains(&(6 as u8)) {
        return 5;
    } else if segments.contains(&(0 as u8))
            && !segments.contains(&(1 as u8))
            && segments.contains(&(2 as u8))
            && segments.contains(&(3 as u8))
            && segments.contains(&(4 as u8))
            && segments.contains(&(5 as u8))
            && segments.contains(&(6 as u8)) {
        return 6;
    } else if segments.contains(&(0 as u8))
            && segments.contains(&(1 as u8))
            && segments.contains(&(2 as u8))
            && !segments.contains(&(3 as u8))
            && !segments.contains(&(4 as u8))
            && !segments.contains(&(5 as u8))
            && !segments.contains(&(6 as u8)) {
        return 7;
    } else if segments.contains(&(0 as u8))
            && segments.contains(&(1 as u8))
            && segments.contains(&(2 as u8))
            && segments.contains(&(3 as u8))
            && segments.contains(&(4 as u8))
            && segments.contains(&(5 as u8))
            && segments.contains(&(6 as u8)) {
        return 8;
    } else if segments.contains(&(0 as u8))
            && segments.contains(&(1 as u8))
            && segments.contains(&(2 as u8))
            && segments.contains(&(3 as u8))
            && !segments.contains(&(4 as u8))
            && segments.contains(&(5 as u8))
            && segments.contains(&(6 as u8)) {
        return 9;
    } else {
        panic!("Invalid digit {:?}", segments);
    }
}

fn find_six_digit<'a>(all_digits: Vec<&'a str>, segments_map: HashMap<u8, char>, digits_map: &mut HashMap<u8, Vec<&'a str>>) {
    let five = *digits_map.get(&5).expect("No five digit").last().expect("No five digit");
    let upper_left_segment = segments_map.get(&1).expect("No upper left segment");
    all_digits.iter().filter(|x|
            x.len() == 6
                    && five.chars().all(|c| x.contains(c))
                    && !x.chars().any(|c| c == upper_left_segment.clone()))
            .for_each(|x| {
                digits_map.get_mut(&6).expect("No six digit").push(x);
            });
}


fn find_five_digit<'a>(all_digits: Vec<&'a str>, segments_map: HashMap<u8, char>, digits_map: &mut HashMap<u8, Vec<&'a str>>) {
    let nine = digits_map.get(&9).expect("No nine digit").last().expect("No nine digit");
    let upper_left_char = segments_map.get(&5).expect("No upper left segment").clone();
    all_digits.iter().filter(|x|
            x.len() == 5
                    && x.chars().all(|c| nine.contains(c))
                    && x.contains(upper_left_char)
    ).for_each(|x| {
        digits_map.get_mut(&5).expect("No five digit").push(x);
    });
}

fn find_three_digit<'a>(all_digits: Vec<&'a str>, digits_map: &mut HashMap<u8, Vec<&'a str>>) {
    let nine = digits_map.get(&9).expect("No nine digit").last().expect("No nine digit");
    let one = digits_map.get(&1).expect("No one digit").last().expect("No one digit");
    all_digits.iter().filter(|x|
            x.len() == 5
                    && x.chars().all(|c| nine.contains(c))
                    && one.chars().all(|c| x.contains(c))
    ).for_each(|x| {
        digits_map.get_mut(&3).expect("No three digit").push(x);
    });
}

fn find_nine_digit<'a>(all_digits: Vec<&'a str>, digits_map: &mut HashMap<u8, Vec<&'a str>>) {
    let digits_map_clone = digits_map.clone();
    let four = digits_map_clone.get(&4).expect("No four digit").last().expect("No four digit");
    let seven = digits_map_clone.get(&7).expect("No seven digit").last().expect("No seven digit");
    all_digits.iter().filter(
        |x| {
            x.len() == 6 && seven.chars().all(|y| x.contains(y)) && four.chars().all(|y| x.contains(y))
        }
    ).for_each(|x| {
        digits_map.get_mut(&9).expect("No nine digit").push(x);
    });
}

/// Finds the upper segment of the 7 segments display and puts it in the segments_map
/// *******
/// |     |
/// |.....|
/// |     |
/// .......
fn find_upper_segment(digits_map: HashMap<u8, Vec<&str>>, segments_map: &mut HashMap<u8, char>) {
    let seven = *digits_map.get(&7).expect("No 7 found").last().expect("No 7 found");
    let one = *digits_map.get(&1).expect("No 1 found").last().expect("No 1 found");
    for c in seven.chars() {
        if !one.chars().any(|x| x == c) {
            segments_map.insert(0, c.clone());
            break;
        }
    }
}

/// Finds the down segment of the 7 segments display and puts it in the segments_map
/// .......
/// |     |
/// |.....|
/// |     |
/// *******
fn find_lower_segment(digits_map: HashMap<u8, Vec<&str>>, segments_map: &mut HashMap<u8, char>) {
    let digits_map_clone = digits_map.clone();
    let four = digits_map_clone.get(&4).expect("No four digit").last().expect("No four digit");
    let seven = digits_map_clone.get(&7).expect("No seven digit").last().expect("No seven digit");
    let nine = digits_map_clone.get(&9).expect("No nine digit").last().expect("No nine digit");
    for c in nine.chars() {
        if !seven.chars().any(|x| x == c) && !four.chars().any(|x| x == c) {
            segments_map.insert(3, c.clone());
            break;
        }
    }
}

/// Finds the upper left segment of the 7 segments display and puts it in the segments_map
/// *......
/// *     |
/// *.....|
/// |     |
/// .......
fn find_upper_left_segment(digits_map: HashMap<u8, Vec<&str>>, segments_map: &mut HashMap<u8, char>) {
    let digits_map_clone = digits_map.clone();
    let nine = digits_map_clone.get(&9).expect("No nine digit").last().expect("No nine digit");
    let three = digits_map_clone.get(&3).expect("No three digit").last().expect("No three digit");
    for c in nine.chars() {
        if !three.chars().any(|x| x == c) {
            segments_map.insert(5, c.clone());
            break;
        }
    }
}

/// Finds the upper left segment of the 7 segments display and puts it in the segments_map
/// ......*
/// |     *
/// |.....*
/// |     |
/// .......
fn find_upper_right_segment(digits_map: HashMap<u8, Vec<&str>>, segments_map: &mut HashMap<u8, char>) {
    let digits_map_clone = digits_map.clone();
    let nine = digits_map_clone.get(&9).expect("No nine digit").last().expect("No nine digit");
    let five = digits_map_clone.get(&5).expect("No five digit").last().expect("No five digit");
    for c in nine.chars() {
        if !five.chars().any(|x| x == c) {
            segments_map.insert(1, c.clone());
            break;
        }
    }
}

/// Finds the upper left segment of the 7 segments display and puts it in the segments_map
/// .......
/// |     |
/// |.....*
/// |     *
/// ......*
fn find_lower_right_segment(digits_map: HashMap<u8, Vec<&str>>, segments_map: &mut HashMap<u8, char>) {
    let digits_map_clone = digits_map.clone();
    let one = digits_map_clone.get(&1).expect("No one digit").last().expect("No one digit");
    let segments_map_clone = segments_map.clone();
    let lower_right_segment = segments_map_clone.get(&1).expect("No lower right segment");
    for c in one.chars() {
        if c != *lower_right_segment {
            segments_map.insert(2, c.clone());
            break;
        }
    }
}

/// Finds the upper left segment of the 7 segments display and puts it in the segments_map
/// .......
/// |     |
/// *******
/// |     |
/// ......|
fn find_middle_segment(digits_map: HashMap<u8, Vec<&str>>, segments_map: &mut HashMap<u8, char>) {
    let digits_map_clone = digits_map.clone();
    let four = digits_map_clone.get(&4).expect("No 4 digit").last().expect("No 4 digit");
    let segments_map_clone = segments_map.clone();
    for c in four.chars() {
        if !segments_map_clone.values().any(|x| *x == c) {
            segments_map.insert(6, c.clone());
            break;
        }
    }
}

/// Finds the upper left segment of the 7 segments display and puts it in the segments_map
/// .......
/// |     |
/// *------
/// *     |
/// *.....|
fn find_lower_left_segment(digits_map: HashMap<u8, Vec<&str>>, segments_map: &mut HashMap<u8, char>) {
    let digits_map_clone = digits_map.clone();
    let six = *digits_map_clone.get(&6).expect("No 6 digit").last().expect("No 6 digit");
    for c in six.chars() {
        if !segments_map.values().any(|x| *x == c) {
            segments_map.insert(4, c.clone());
            break;
        }
    }
}

fn find_unique_digits<'a>(digits: Vec<&'a str>, digits_map: &mut HashMap<u8, Vec<&'a str>>) {
    for digit in digits {
        match digit.len() {
            2 => {
                digits_map.get_mut(&(1 as u8)).expect("error reading the map").push(digit.clone());
            }
            3 => {
                digits_map.get_mut(&(7 as u8)).expect("error reading the map").push(digit.clone());
            }
            4 => {
                digits_map.get_mut(&(4 as u8)).expect("error reading the map").push(digit.clone());
            }
            7 => {
                digits_map.get_mut(&(8 as u8)).expect("error reading the map").push(digit.clone());
            }
            _ => {}
        };
    }
}

fn parse_digits(input: &str) -> Vec<&str> {
    input.split(" ")
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>()
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