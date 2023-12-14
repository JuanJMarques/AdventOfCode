use std::fs::File;
use std::io::Read;

fn main() {
    let extrapolation_sum = process_sensor(read_input_file());
    println!("the sum of extrapolations is {}", extrapolation_sum);
}

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}


fn process_sensor(input: String) -> i32{
    input.lines()
        .map(parse_input)
        .map(extrapolate)
        .sum()

}

fn extrapolate(history: Vec<i32>) -> i32 {
    let mut matrix = vec![history];
    while !matrix.last().unwrap().iter().all(|&data| data == 0) {
        let mut new_row = Vec::new();
        let last_row = matrix.last().unwrap();
        for (i, &elem) in last_row.iter().enumerate(){
            if i != 0 {
                new_row.push(elem - last_row[i-1])
            }
        }
        matrix.push(new_row);
    }
    let mut extrapolation = 0;
    while !matrix.is_empty() {
        extrapolation += matrix.pop().unwrap().pop().unwrap();
    }
    extrapolation
}

fn parse_input(line: &str) -> Vec<i32> {
    line.split(' ')
        .map(|s| s.trim())
        .filter(|&s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            .to_string();
        let result = process_sensor(input);
        assert_eq!(result, 114);
    }
}