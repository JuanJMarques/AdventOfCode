use std::fs::File;
use std::io::Read;

fn main() {
    let sum = sum_rounded_rocks(read_input_file());
    println!("the sum of rounded rocks is {}", sum);
}

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}

fn sum_rounded_rocks(input: String) -> u32 {
    let mut lines = Vec::new();
    for line in input.lines() {
        lines.push(line);
    }
    let lines = tilt_platform(lines);
    let lines = lines.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    let lines = flip(lines);
    lines.iter().enumerate().map(|(i, &line)| (i as u32 + 1) * count_rounded_rocks(line)).sum()
}

fn tilt_platform(lines: Vec<&str>) -> Vec<String> {
    let mut lines = lines.clone().iter().map(|s| s.to_string()).collect::<Vec<String>>();
    let mut old_lines = Vec::new();
    while old_lines != lines {
        old_lines = lines.clone();
        lines = tilt(lines);
    }
    lines
}

fn tilt(lines: Vec<String>) -> Vec<String> {
    let lines = lines.iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut new_lines = lines.clone();
    for i in 1..new_lines.len() {
        for j in 0..new_lines[i].len() {
            if new_lines[i][j] == 'O' && new_lines[i-1][j] == '.' {
                new_lines[i-1][j] = 'O';
                new_lines[i][j] = '.';
            }
        }
    }
    new_lines.iter().map(|v| v.iter().copied().collect::<String>()).collect::<Vec<String>>()
}

fn count_rounded_rocks(line: &str) -> u32 {
    line.chars().filter(|&c| c == 'O').count() as u32
}


fn flip(lines: Vec<&str>) -> Vec<&str> {
    let mut flipped = vec![""; lines.len()];
    for (i, &line) in lines.iter().enumerate() {
        flipped[lines.len() - (1 + i)] = line;
    }
    flipped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
            .to_string();
        let result = sum_rounded_rocks(input);
        assert_eq!(result, 136);
    }
}
