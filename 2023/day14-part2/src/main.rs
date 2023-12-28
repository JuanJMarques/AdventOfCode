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
        lines.push(line.to_string());
    }
    let lines = tilt_platform_n_times(lines, 1000000000);
    let lines = lines.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    let lines = flip(lines);
    lines.iter().enumerate().map(|(i, &line)| (i as u32 + 1) * count_rounded_rocks(line)).sum()
}

fn tilt_platform_n_times(lines: Vec<String>, times: u32) -> Vec<String> {
    let mut lines = lines.clone();
    let mut history = Vec::new();
    for i in 1..times+1 {
        if times % i == 0 {
            lines = tilt_platform(lines.clone());
            if history.contains(&lines.clone()) {
                return lines;
            }
            history.push(lines.clone())
        }

    }
    lines
}

fn tilt_platform(lines: Vec<String>) -> Vec<String> {
    let mut lines = lines.clone();
    let mut old_lines = Vec::new();
    while old_lines != lines {
        old_lines = lines.clone();
        lines = tilt_north(lines);
    }
    old_lines = Vec::new();
    while old_lines != lines {
        old_lines = lines.clone();
        lines = tilt_west(lines);
    }
    old_lines = Vec::new();
    while old_lines != lines {
        old_lines = lines.clone();
        lines = tilt_south(lines);
    }
    old_lines = Vec::new();
    while old_lines != lines {
        old_lines = lines.clone();
        lines = tilt_east(lines);
    }
    lines
}


fn tilt_north(lines: Vec<String>) -> Vec<String> {
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

fn tilt_west(lines: Vec<String>) -> Vec<String> {
    let lines = lines.iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut new_lines = lines.clone();
    for i in 0..new_lines.len() {
        for j in 1..new_lines[i].len() {
            if new_lines[i][j] == 'O' && new_lines[i][j-1] == '.' {
                new_lines[i][j] = '.';
                new_lines[i][j-1] = 'O';
            }
        }
    }
    new_lines.iter().map(|v| v.iter().copied().collect::<String>()).collect::<Vec<String>>()
}

fn tilt_south(lines: Vec<String>) -> Vec<String> {
    let lines = lines.iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut new_lines = lines.clone();
    for i in 0..new_lines.len() - 1 {
        let i = new_lines.len()-(2+i);
        for j in 0..new_lines[i].len() {
            if new_lines[i][j] == 'O' && new_lines[i+1][j] == '.' {
                new_lines[i+1][j] = 'O';
                new_lines[i][j] = '.';
            }
        }
    }
    new_lines.iter().map(|v| v.iter().copied().collect::<String>()).collect::<Vec<String>>()
}

fn tilt_east(lines: Vec<String>) -> Vec<String> {
    let lines = lines.iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut new_lines = lines.clone();
    for i in 0..new_lines.len() {
        for j in 0..new_lines[i].len()-1 {
            let j = new_lines[i].len()-(2+j);
            if new_lines[i][j+1] == '.' && new_lines[i][j] == 'O' {
                new_lines[i][j] = '.';
                new_lines[i][j+1] = 'O';
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
        assert_eq!(result, 64);
    }

    #[test]
    fn test_2 () {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....".to_string();
        let mut vec_input = Vec::new();
        for line in input.lines() {
            vec_input.push(line.to_string());
        }
        let output = tilt_platform_n_times(vec_input,1);

        let expected = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....".to_string();
        let mut vec_expected = Vec::new();
        for line in expected.lines(){
            vec_expected.push(line.to_string());
        }
        assert_eq!(output,vec_expected)
    }

    #[test]
    fn test_3 () {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....".to_string();
        let mut vec_input = Vec::new();
        for line in input.lines() {
            vec_input.push(line.to_string());
        }
        let output = tilt_platform(vec_input);
        let output = tilt_platform(output);
        let output = tilt_platform(output);

        let expected = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O".to_string();
        let mut vec_expected = Vec::new();
        for line in expected.lines(){
            vec_expected.push(line.to_string());
        }
        assert_eq!(output,vec_expected)
    }

}
