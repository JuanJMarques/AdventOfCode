mod lib;


use std::fs::File;
use std::io::Read;
use crate::lib::connect_junction_boxes;

fn main() {
    println!("The multiplication of the three largest circuits is {}", connect_junction_boxes(read_input_file(), 1000));
}

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}



