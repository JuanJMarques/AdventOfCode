use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Clone,Debug)]
struct Len {
    label: String,
    focal_length: u32,
}

fn main() {
    let sum = sum_focusing_power(read_input_file());
    println!("the sum of focusing powers is {}", sum);
}

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}

fn sum_focusing_power(input: String) -> u32 {
    let mut boxes = HashMap::with_capacity(256);
    for operand in input.split(',') {
        let operand = operand.trim().to_string();
        boxes = apply_operand(operand.clone(), boxes);
    }
    boxes.iter()
        .map(|(&i,lens)| lens.iter().enumerate().map(|(j,len)| (i as u32 + 1) * (j as u32 + 1) * len.focal_length).sum::<u32>())
        .sum()
}

fn apply_operand(operand: String, boxes: HashMap<u8, Vec<Len>>) -> HashMap<u8, Vec<Len>> {
    let mut boxes = boxes.clone();
    if operand.contains('=') {
        let parts = operand.split('=').map(|s| s.trim().to_string()).collect::<Vec<String>>();
        let len = Len {
            label: parts[0].clone(),
            focal_length: parts[1].parse::<u32>().unwrap(),
        };
        let mut lens = Vec::new();
        let label = len.label.clone();
        let hash = get_hash(label.as_str());
        if boxes.contains_key(&hash) {
            lens = boxes.get(&hash).unwrap().clone();
            let index = lens.iter().enumerate()
                .filter(|(_, l)| l.label == label)
                .map(|(i, _)| i)
                .next();
            if let Some(i) = index {
                lens.remove(i);
                lens.insert(i, len);
            }else {
                lens.push(len);
            }
        }else {
            lens.push(len);
        }
        boxes.insert(hash, lens);
    } else if operand.ends_with('-') {
        let label = operand.replace('-', "");
        let hash = get_hash(label.as_str());
        if boxes.contains_key(&hash) {
            let mut lens = boxes.get(&hash).unwrap().clone();
            let index = lens.iter().enumerate()
                .filter(|(_, l)| l.label == label)
                .map(|(i, _)| i)
                .next();
            if let Some(i) = index {
                lens.remove(i);
                boxes.insert(hash, lens);
            }
        }
    }
    boxes
}


fn get_hash(input: &str) -> u8 {
    let mut hash: u8 = 0;
    let mut _ignore: bool;
    for c in input.chars() {
        (hash, _ignore) = hash.overflowing_add(c as u8);
        (hash, _ignore) = hash.overflowing_mul(17);
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
        let result = sum_focusing_power(input);
        assert_eq!(result, 145);
    }
}