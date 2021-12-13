use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");
    let illegal_character_points_table = create_illegal_character_points_table();
    let mut points = 0;
    for line in contents.lines() {
        let mut chunks_stack = vec![];
        for char in line.chars() {
            match char {
                '<' | '(' | '{' | '[' => {
                    chunks_stack.push(char);
                },
                '>' | ')' | '}' | ']' => {
                    let last_chunk = chunks_stack.pop().unwrap();
                    if !legal_chunk_close(last_chunk, char) {
                        let char_points = illegal_character_points_table.get(&char).unwrap();
                        println!("Illegal chunk close: {} {} with {} points", last_chunk, char, char_points);
                        points += char_points;
                        break;
                    }
                },
                _ => {}
            }
        }
    }
    println!("Points: {}", points);
}

fn legal_chunk_close(open_char: char, close_char: char) -> bool {
    match open_char {
        '<' => {
            close_char == '>'
        },
        '(' => {
            close_char == ')'
        },
        '{' => {
            close_char == '}'
        },
        '[' => {
            close_char == ']'
        },
        _ => false
    }
}

fn create_illegal_character_points_table() -> HashMap<char, u32> {
    let mut table: HashMap<char, u32> = HashMap::new();
    table.insert(')', 3);
    table.insert(']', 57);
    table.insert('}', 1197);
    table.insert('>', 25137);
    table
}

