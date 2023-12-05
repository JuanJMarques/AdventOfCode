use std::fs::File;
use std::io::Read;

fn main() {
    let scratchcards = sum_scratchcards(read_input_file());
    println!("The sum scratchcards amount is {}", scratchcards)
}

fn sum_scratchcards(lottery_results: String) -> usize {
    let lines = lottery_results.lines().collect::<Vec<&str>>();
    let mut copies = std::iter::repeat(1)
        .take(lines.len())
        .collect::<Vec<usize>>();
    for (i, &line) in lines.iter().enumerate() {
        let numbers = line.split(':').collect::<Vec<&str>>();
        let numbers = numbers[1].split('|').collect::<Vec<&str>>();
        let winning_numbers = numbers[0]
            .split(' ')
            .map(|number| number.trim())
            .filter(|number| !number.is_empty())
            .collect::<Vec<&str>>();
        let my_numbers = numbers[1]
            .split(' ')
            .map(|number| number.trim())
            .filter(|number| !number.is_empty())
            .collect::<Vec<&str>>();
        let mut counter = 0;
        for number in my_numbers.clone() {
            for winning_number in winning_numbers.clone() {
                if winning_number == number {
                    counter += 1;
                }
            }
        }
        for j in 1..(counter + 1) {
            if i + j < lines.len() {
                copies[i + j] += copies[i];
            }
        }
    }
    copies.iter().sum()
}

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .to_string();
        let result = sum_scratchcards(input);
        assert_eq!(result, 30);
    }
}
