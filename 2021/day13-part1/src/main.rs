use std::collections::HashSet;
use std::fs::{File};
use std::io::{Read};


fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");
    println!("Total paths: {}", calculate_visible_dots(contents));
}

fn calculate_visible_dots(contents: String) -> u32 {
    let (mut dots_cords, instructions) = parse_input(contents);
    let mut execute = true;
    for instruction in instructions {
        if execute {
            execute_instruction(&mut dots_cords, instruction);
            dots_cords = filter_dots(dots_cords);
            dots_cords.dedup();
            execute = false;
        }
    }
    print_dots(dots_cords.clone());
    dots_cords.len() as u32
}

fn print_dots(dots_cords: Vec<(i32, i32)>) {
    let rows = dots_cords.iter().map(|(_x, y)| y).max().unwrap() + 1;
    let columns = dots_cords.iter().map(|(x, _y)| x).max().unwrap() + 1;
    for y in 0..rows {
        for x in 0..columns {
            if dots_cords.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn filter_dots(mut dots: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    //filter duplicates
    let set: HashSet<_> = dots.drain(..).collect();
    dots.extend(set.into_iter());
    //filter out of the screen
    dots.iter().filter(|dot| dot.0 >= 0 && dot.1 >= 0).map(|dot| *dot).collect()
}

fn execute_instruction(dot_cords: &mut Vec<(i32, i32)>, instruction: (String, i32)) {
    let value = instruction.1;
    for dot in dot_cords.iter_mut() {
        match instruction.0.as_ref() {
            "x" => {
                if dot.0 >= value {
                    if dot.0 == value {
                        dot.0 = -1;
                    } else {
                        dot.0 = value - (dot.0 - value);
                    }
                }
            }
            "y" => {
                if dot.1 >= value {
                    if dot.1 == value {
                        dot.1 = -1;
                    } else {
                        dot.1 = value - (dot.1 - value);
                    }
                }
            }
            _ => panic!("Unknown instruction: {}", instruction.0),
        }
    }
}


fn parse_input(table_str: String) -> (Vec<(i32, i32)>, Vec<(String, i32)>) {
    let mut dots_cords = vec![];
    let mut instructions = vec![];
    let mut parse_instructions = false;
    for line in table_str.lines() {
        if line.trim().is_empty() {
            parse_instructions = true;
            continue;
        }
        if parse_instructions {
            let line = line.replace("fold along ", "");
            let mut instruction = line.split('=');
            let axis = instruction.next().unwrap().to_string();
            let val = instruction.next().unwrap().trim().parse::<i32>().unwrap();
            instructions.push((axis, val));
        } else {
            let mut cords = line.split(',');
            let x = cords.next().unwrap().trim().parse::<i32>().unwrap();
            let y = cords.next().unwrap().trim().parse::<i32>().unwrap();
            dots_cords.push((x, y))
        }
    }
    (dots_cords, instructions)
}

#[test]
fn test_case_day13_1() {
    let points = calculate_visible_dots(
        String::from("6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"));
    assert_eq!(points, 17)
}

