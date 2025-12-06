mod lib;


use std::fs::File;
use std::io::Read;
use crate::lib::find_accessible_rolls;

fn main() {
    println!("the number of accessible rolls is: {}",find_accessible_rolls(read_input_file()));
}

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}



