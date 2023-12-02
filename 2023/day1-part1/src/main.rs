use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    let total = sum_digits(content);
    println!("the total of the digist is {}", total);
}

fn sum_digits(content: String) -> u32 {
    let mut total = 0;
    for line in content.lines() {
        let mut is_first_digit = true;
        let mut first_digit = 0;
        let mut last_digit = 0;
        for char in line.chars() {
            if char.is_digit(10) {
                if is_first_digit {
                    first_digit = char.to_digit(10).unwrap();
                    is_first_digit = false;
                }
                last_digit = char.to_digit(10).unwrap();
            }
        }
        total = total + first_digit * 10 + last_digit;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = format!(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
        );
        let result = sum_digits(input);
        assert_eq!(result, 142);
    }
}