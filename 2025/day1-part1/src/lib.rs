pub fn find_password_by_dial(input: String) -> u32 {
    let mut dial: u8 = 50;
    let mut password : u32 = 0;
    for line in input.lines() {
        let mut multiplier : i8 = 1;
        if line.starts_with('L') {
            multiplier = -1;
        }
        let ammount_str = &line[1..];
        let ammount = ammount_str.parse::<i32>().unwrap() % 100;
        let ammount = (((multiplier as i32 * ammount) + 100) % 100) as u8;
        dial = (dial + ammount) % 100;
        // dial = dial.abs() % 100;
        if dial == 0 {
            password += 1;
        }
    }
    password
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
            .to_string();
        let result = find_password_by_dial(input);
        assert_eq!(result, 3);
    }
}