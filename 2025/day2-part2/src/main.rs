mod lib;

use std::fs::File;
use std::io::Read;
use lib::sum_valid_ids;
fn main() {
    println!("the sum of valid ids is: {}",sum_valid_ids(read_input_file()));
}

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}



