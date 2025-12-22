mod lib;


use std::fs::File;
use std::io::Read;
use crate::lib::count_beam_splits;

fn main() {
    println!("The tachyon ends up in {} different timelines", count_beam_splits(read_input_file()));
}

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}



