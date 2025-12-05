
pub fn sum_joltage(input: String) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut left_max = 0;
        let mut right_max = 0;
        for ch in line.chars() {
            if ch.is_digit(10) {
                let digit = ch.to_digit(10).unwrap();
                if left_max == 0 {
                    left_max = digit;
                    continue;
                }
                if right_max == 0 {
                    right_max = digit;
                    continue;
                }
                if right_max > left_max {
                    left_max = right_max;
                    right_max = digit;
                    continue;
                }
                if digit > right_max {
                    right_max = digit;
                }
            }
        }
        sum += left_max * 10 + right_max
    }
    sum
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
        assert_eq!(result, 357);
    }
}