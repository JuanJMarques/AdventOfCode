use std::collections::HashMap;
use std::fs::{File};
use std::io::{Read};

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");
    println!("Points: {}", calculate_missing_points(contents));
}

fn calculate_missing_points(contents: String) -> u64 {
    let missing_character_points_table = create_missing_character_points_table();
    let mut points = vec![];
    for line in contents.lines() {
        let mut chunks_stack = vec![];
        let mut correct_line = true;
        for char in line.chars() {
            match char {
                '<' | '(' | '{' | '[' => {
                    chunks_stack.push(char);
                }
                '>' | ')' | '}' | ']' => {
                    let last_chunk = chunks_stack.pop().unwrap();
                    if !legal_chunk_close(last_chunk, char) {
                        correct_line = false;
                        break;
                    }
                }
                _ => {}
            }
        }
        if correct_line {
            let mut line_points: u64 = 0;
            chunks_stack.reverse();
            for chunk in chunks_stack {
                line_points = line_points * 5 + *missing_character_points_table.get(&get_correct_chunk_close(chunk)).unwrap() as u64;
            }
            points.push(line_points);
        }
    }
    points.sort();
    //get the middle element of the sorted array
    if points.len() % 2 == 0 {
        points[(points.len() / 2) - 1]
    } else {
        points[points.len() / 2]
    }
}

fn get_correct_chunk_close(open_char: char) -> char {
    match open_char {
        '<' => {
            '>'
        }
        '(' => {
            ')'
        }
        '{' => {
            '}'
        }
        '[' => {
            ']'
        }
        _ => panic!("Unknown open char: {}", open_char)
    }
}

fn legal_chunk_close(open_char: char, close_char: char) -> bool {
    match open_char {
        '<' => {
            close_char == '>'
        }
        '(' => {
            close_char == ')'
        }
        '{' => {
            close_char == '}'
        }
        '[' => {
            close_char == ']'
        }
        _ => panic!("Unknown open char: {}", open_char)
    }
}

fn create_missing_character_points_table() -> HashMap<char, u32> {
    let mut table: HashMap<char, u32> = HashMap::new();
    table.insert(')', 1);
    table.insert(']', 2);
    table.insert('}', 3);
    table.insert('>', 4);
    table
}

#[test]
fn test_case() {
    let points = calculate_missing_points(String::from("[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"));
    assert_eq!(points, 288957)
}