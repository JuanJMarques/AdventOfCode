use std::fmt::format;

pub fn sum_valid_ids(input: String) -> u64 {
    let mut valid_ids: Vec<u64> = vec![];
    for line in input.lines() {
        for range in line.split(',') {
            let limits = get_limits(range);
            valid_ids.extend(get_valid_ids(limits[0], limits[1]));
        }
    }
    valid_ids.iter().sum()
}

fn get_valid_ids(inf: u64, sup: u64) -> Vec<u64> {
    let mut valid_ids: Vec<u64> = vec![];
    let digits_amount = inf.checked_ilog10().unwrap()+1;
    let div_point = digits_amount / 2; //+ if digits_amount % 2 != 0 { 1 } else { 0 };
    let div_point = if div_point == 0 {1} else {div_point};
    let half = inf.to_string();
    let half = half.as_str();
    let mut half = half[0..div_point as usize].parse::<u64>().unwrap();
    let mut actual = format!("{}{}", half, half).parse::<u64>().unwrap();
    while actual <= sup {
        if actual >= inf {
            valid_ids.push(actual);
        }
        half += 1;
        actual = format!("{}{}", half, half).parse::<u64>().unwrap();
    }
    valid_ids
}

fn get_limits(line: &str) -> Vec<u64> {
    line.trim().split('-').collect::<Vec<&str>>()
        .iter().map( |&s| s.trim().parse::<u64>().unwrap()).collect::<Vec<u64>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = "1-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            .to_string();
        let result = sum_valid_ids(input);
        assert_eq!(result, 1227775554);
    }
}