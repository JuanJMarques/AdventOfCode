use std::fs::File;
use std::io::Read;

fn main() {
    let part_numbers_sum = sum_gear_ratios(read_input_file());
    println!("The sum part gear ratios {}", part_numbers_sum)
}

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}

fn sum_gear_ratios(engine_schematic: String) -> u32 {
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
    let mut gear_ratio_sum = 0;
    for (i, line) in lines_vec.iter().enumerate() {
        for (j, &char) in line.iter().enumerate() {
            if char.to_ascii_lowercase() == '*'.to_ascii_lowercase() {
                let candidates = get_gear_ratio_candidates((i, j), (max_i, max_j), &lines_vec);
                if candidates.len() == 2 {
                    gear_ratio_sum += candidates.iter().product::<u32>()
                }
            }
        }
    }
    gear_ratio_sum
}

fn get_gear_ratio_candidates(
    curr_cords: (usize, usize),
    limit: (usize, usize),
    lines: &[Vec<char>],
) -> Vec<u32> {
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
    let mut candiates = vec![];
    for i in despl_i {
        let new_i = (curr_cords_i as i32 + i) as usize;
        let line = lines.get(new_i).unwrap();
        for &j in &despl_j {
            let new_j = (curr_cords_j as i32 + j) as usize;
            let &char = line.get(new_j).unwrap();
            if char.is_ascii_digit() {
                let candidate = get_candidate((new_i, new_j), limit, line);
                if !candiates.contains(&candidate) && candidate != 0 {
                    candiates.push(candidate)
                }
            }
        }
    }
    candiates
}

fn get_candidate(coords: (usize, usize), limit: (usize, usize), line: &[char]) -> u32 {
    let (_, coords_j) = coords;
    let (_, limit_j) = limit;
    let mut coords_j_mut = coords_j;
    let mut candidate = 0;
    let mut pow = 1;
    let mut should_continue = true;
    while should_continue {
        let candidate_char = line.get(coords_j_mut).unwrap();
        if candidate_char.is_ascii_digit() {
            candidate += pow * candidate_char.to_digit(10).unwrap();
            pow *= 10;
        } else {
            should_continue = false;
        }
        if coords_j_mut > 0 {
            coords_j_mut -= 1;
        } else {
            should_continue = false;
        }
    }
    coords_j_mut = coords_j + 1;
    should_continue = coords_j_mut < limit_j;
    while should_continue {
        let candidate_char = line.get(coords_j_mut).unwrap();
        if candidate_char.is_ascii_digit() {
            candidate = candidate * 10 + candidate_char.to_digit(10).unwrap();
        } else {
            should_continue = false
        }
        coords_j_mut += 1;
        should_continue = should_continue && coords_j_mut < limit_j;
    }
    candidate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .to_string();
        let result = sum_gear_ratios(input);
        assert_eq!(result, 467835);
    }
}
