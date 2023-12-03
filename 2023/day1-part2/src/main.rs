use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    let total = sum_digits(content);
    println!("the total of the digits is {}", total);
}

fn sum_digits(content: String) -> u32 {
    let mut total = 0;
    for line in content.lines() {
        let mut is_first_digit = true;
        let mut first_digit = 0;
        let mut last_digit = 0;
        let mut buffer = String::new();
        for char in line.chars() {
            buffer.push(char);
            // a digit can only be 5 letter length spelled
            if buffer.len() > 5 {
                let _ = buffer.remove(0);
            }
            if char.is_ascii_digit() {
                if is_first_digit {
                    first_digit = char.to_digit(10).unwrap();
                    is_first_digit = false;
                }
                last_digit = char.to_digit(10).unwrap();
                buffer.clear();
            } else if contains_digit_spelled(buffer.clone()) {
                if is_first_digit {
                    first_digit = parse_digit_spelled(buffer.clone());
                    is_first_digit = false;
                }
                last_digit = parse_digit_spelled(buffer.clone());
            }
        }
        total = total + first_digit * 10 + last_digit;
    }
    total
}

fn contains_digit_spelled(text: String) -> bool {
    text.contains("one")
        || text.contains("two")
        || text.contains("three")
        || text.contains("four")
        || text.contains("five")
        || text.contains("six")
        || text.contains("seven")
        || text.contains("eight")
        || text.contains("nine")
}

fn parse_digit_spelled(text: String) -> u32 {
    if text.contains("one") {
        return 1;
    }
    if text.contains("two") {
        return 2;
    }
    if text.contains("three") {
        return 3;
    }
    if text.contains("four") {
        return 4;
    }
    if text.contains("five") {
        return 5;
    }
    if text.contains("six") {
        return 6;
    }
    if text.contains("seven") {
        return 7;
    }
    if text.contains("eight") {
        return 8;
    }
    if text.contains("nine") {
        return 9;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input =
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen".to_string();
        let result = sum_digits(input);
        assert_eq!(result, 281);
    }
}
