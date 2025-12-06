
pub fn sum_joltage(input: String) -> u64 {
    let mut sum = 0;
    let mut i = 1.0;
    for line in input.lines() {
        sum += find_maximum_joltage_with_batteries(line);
        i+=1.0;
    }
    sum
}

fn find_maximum_joltage_with_batteries(batteries: &str) -> u64 {
    let mut actual_jolatge :u64 = 0;
    for i in 0..batteries.len() {
        let x = batteries[i..i+1].parse::<u64>().unwrap();
        while actual_jolatge > 0
            && x > (actual_jolatge % 10)
            && (actual_jolatge.checked_ilog10().unwrap_or(0) + (batteries.len() - i) as u32) >= 12 {
            actual_jolatge -= (actual_jolatge % 10);
            actual_jolatge /= 10;
        }
        let used_batteries = actual_jolatge.checked_ilog10().unwrap_or(0) + 1;
        if used_batteries < 12 {
            actual_jolatge = actual_jolatge * 10 + x;
        }
    }
    actual_jolatge
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111"
            .to_string();
        let result = sum_joltage(input);
        assert_eq!(result, 3121910778619);
    }
}