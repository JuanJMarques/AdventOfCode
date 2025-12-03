pub fn find_password_by_dial(input: String) -> u32 {
    let mut dial: i32 = 50;
    let mut password : u32 = 0;
    for line in input.lines() {
        let mut multiplier : i32 = 1;
        if line.starts_with('L') {
            multiplier = -1;
        }
        let ammount_str = &line[1..];
        let ammount = ammount_str.parse::<i32>().unwrap();
        if ammount >= 100 {
            password += (ammount / 100) as u32;
        }
        let ammount = ammount % 100;
        let ammount = multiplier * ammount;
        if dial != 0 && (dial + ammount >= 100 || dial + ammount <= 0) {
            password += 1;
        }
        dial = (dial + ammount + 100) % 100;
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
L82
R1001"
            .to_string();
        let result = find_password_by_dial(input);
        assert_eq!(result, 16);
    }
}