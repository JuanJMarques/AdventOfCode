mod lib;


use std::fs::File;
use std::io::Read;
use crate::lib::do_homework;

fn main() {
    println!("the grand total is: {}", do_homework(read_input_file()));
}

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}



