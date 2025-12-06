mod lib;

use std::fs::File;
use std::io::Read;
use crate::lib::sum_joltage;

fn main() {
    println!("the total output joltaje is: {}",sum_joltage(read_input_file()));
}

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}



