use fancy_regex::Regex;

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
    let regex = Regex::new(r"^(\d+)\1+$").unwrap();

    (inf..=sup).filter(|x| regex.is_match(format!("{}",x).as_str()).unwrap()).collect::<Vec<u64>>()
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
        assert_eq!(result, 4174379265);
    }
}