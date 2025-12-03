mod lib;

use std::fs::File;
use std::io::Read;
use lib::find_password_by_dial;
fn main() {
    println!("password: {}",find_password_by_dial(read_input_file()));
}

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}



