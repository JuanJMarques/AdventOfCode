use std::fs::File;
use std::io::Read;

fn main() {
    let part_numbers_sum = sum_part_numbers(read_input_file());
    println!("The sum part numbers is {}", part_numbers_sum)
}

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}

fn sum_part_numbers(engine_schematic: String) -> u32 {
    let mut max_i = 0;
    let mut max_j = 0;
    let mut lines_vec = vec![];
    for (i, line) in engine_schematic.lines().enumerate() {
        let mut line_vec = vec![];
        for (j, char) in line.chars().enumerate() {
            line_vec.push(char);
            max_j = j;
        }
        lines_vec.push(line_vec);
        max_i = i;
    }
    max_i += 1;
    max_j += 1;
    let mut part_numbers_vec = vec![];
    for (i, line) in lines_vec.iter().enumerate() {
        let mut curr_number = 0;
        let mut curr_num_has_adjacent_symbol = false;
        for (j, &char) in line.iter().enumerate() {
            if char.is_ascii_digit() {
                curr_num_has_adjacent_symbol = curr_num_has_adjacent_symbol || has_adjacent_symbol((i, j), (max_i, max_j), &lines_vec);
                curr_number = curr_number * 10 + char.to_digit(10).unwrap();
            } else {
                if curr_num_has_adjacent_symbol {
                    part_numbers_vec.push(curr_number);
                }
                curr_number = 0;
                curr_num_has_adjacent_symbol = false;
            }
        }
        if curr_num_has_adjacent_symbol {
            part_numbers_vec.push(curr_number);
        }
    }
    part_numbers_vec.iter().sum()
}

fn has_adjacent_symbol(curr_cords: (usize, usize), limit: (usize, usize), lines: &[Vec<char>]) -> bool {
    let (curr_cords_i, curr_cords_j) = curr_cords;
    let (limit_i, limit_j) = limit;
    let mut despl_i: Vec<i32> = vec![];
    if curr_cords_i > 0 {
        despl_i.push(-1);
    }
    despl_i.push(0);
    if curr_cords_i + 1 < limit_i {
        despl_i.push(1);
    }
    let mut despl_j: Vec<i32> = vec![];
    if curr_cords_j > 0 {
        despl_j.push(-1);
    }
    despl_j.push(0);
    if curr_cords_j + 1 < limit_j {
        despl_j.push(1);
    }
    let mut has_adjacent_symbol = false;
    for i in despl_i {
        let new_i = (curr_cords_i as i32 + i) as usize;
        let line = lines.get(new_i).unwrap();
        for &j in &despl_j {
            let new_j = (curr_cords_j as i32 + j) as usize;
            let &char = line.get(new_j).unwrap();
            has_adjacent_symbol = has_adjacent_symbol
                || !(char.is_ascii_digit() || char.to_ascii_lowercase() == '.'.to_ascii_lowercase());
        }
    }
    has_adjacent_symbol
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input =
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..".to_string();
        let result = sum_part_numbers(input);
        assert_eq!(result, 4361);
    }
}
