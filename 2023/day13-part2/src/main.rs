use std::fs::File;
use std::io::Read;

fn main() {
    let sum = sum_reflection_points(read_input_file());
    println!("the sum of reflection points is {}", sum);
}

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}

fn sum_reflection_points(input: String) -> u32 {
    let mut buffer= String::new();
    let mut sum = 0;
    for line in input.lines(){
        if line.trim().is_empty() {
            sum += get_reflection_point(buffer.as_str());
            buffer = String::new();
        }else if buffer.is_empty() {
            buffer = line.to_string();
        }else {
            buffer = format!("{}\n{}", buffer.clone(), line.clone());
        }
    }
    if !buffer.is_empty() {
        sum += get_reflection_point(buffer.as_str());
    }
    sum
}

fn get_reflection_point(buffer: &str) -> u32 {
    let mut hor_buffer = vec![];
    for line in buffer.lines(){
        hor_buffer.push(line);
    }
    let point_h = search_reflection_point(hor_buffer.clone());
    let mut point_v = 0;
    if point_h == 0 {
        let vec_buffer = transpose(hor_buffer.clone());
        let vec_buffer = vec_buffer.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
        point_v = search_reflection_point(vec_buffer);
    }
    point_h * 100 + point_v
}

fn count_different_chars(buff1: &str, buff2: &str) -> u32 {
    let buff1 = buff1.chars().collect::<Vec<char>>();
    let buff2 = buff2.chars().collect::<Vec<char>>();
    let mut count = 0;
    for i in 0..buff1.len(){
        if  buff1[i] != buff2[i] {
            count += 1;
        }
    }
    count
}

fn search_reflection_point(buffer: Vec<&str>) -> u32 {
    for i in 0..buffer.len() - 1{
        if check_candidate(i, buffer.clone()) {
            return (i + 1) as u32;
        }
    }
    0
}

fn check_candidate(i: usize, buffer: Vec<&str>) -> bool {
    let mut up = i as i32;
    let mut down = 1;
    let mut smudged_rows = false;
    while up >= 0 && i+down < buffer.len() {
        if buffer[up as usize] != buffer[i+down]{
            if smudged_rows {
                return false
            }else {
                let num_differences = count_different_chars(buffer[up as usize], buffer[i+down]);
                if num_differences == 1 {
                    smudged_rows = true;
                }else {
                    return false;
                }
            }

        }
        up -= 1;
        down += 1;
    }
    true
}

fn transpose(board: Vec<&str>) -> Vec<String> {
    let mut transposed_board = vec![vec![' '; board.len()]; board[0].len()];
    for (row_index, &row) in board.iter().enumerate() {
        for (column_index, c) in row.chars().enumerate() {
            transposed_board[column_index][row_index] = c;
        }
    }
    let mut trasposed = vec![];
    for line in transposed_board {
        trasposed.push(line.iter().collect::<String>())
    }
    trasposed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
            .to_string();
        let result = sum_reflection_points(input);
        assert_eq!(result, 400);
    }
}