use std::fs::File;
use std::io::Read;

fn main() {
    let sum = sum_hashes(read_input_file());
    println!("the sum of rounded rocks is {}", sum);
}

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}

fn sum_hashes(input: String) -> u32 {
    input.split(',').map(|s| get_hash(s.trim()) as u32).sum()
}

fn get_hash(input: &str) -> u8 {
    let mut hash:u8 = 0;
    let mut _ignore : bool;
    for c in input.chars(){
        (hash,_ignore) = hash.overflowing_add(c as u8);
        (hash,_ignore) = hash.overflowing_mul(17);
    }
    hash
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
            .to_string();
        let result = sum_hashes(input);
        assert_eq!(result, 1320);
    }
}