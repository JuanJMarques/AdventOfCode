use std::fs::File;
use std::io::Read;

fn main() {
    let mut previous_measure: Option<i32> = None;
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let mut position = (0, 0);
    println!("Starting moving");
    for line in contents.lines() {
        println!("actual Position: {}, {}", position.0, position.1);
        println!("moving {}", line);
        match parse_course(line) {
            ("forward", x) => {
                position.0 += x;
            }
            ("up", x) => {
                position.1 -= x;
            }
            ("down", x) => {
                position.1 += x;
            }
            _ => {
                panic!("Unknown course");
            }
        }
    }
    println!("actual Position: {}, {}", position.0, position.1);
    println!("multiplied coordinates {}", position.0 * position.1);
}

fn parse_course(line: &str) -> (&str, i32) {
    let mut parts = line.split(" ");
    let direction = parts.next().unwrap();
    let distance = parts.next().unwrap().parse::<i32>().unwrap();
    (direction, distance)
}