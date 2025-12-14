pub fn do_homework(input: String) -> u64 {
    let lines = input.lines();
    let total_lines :usize = lines.clone().count();
    let mut numbers = vec![];
    let mut operators: Vec<&str> = vec![];
    for (i,line) in lines.enumerate() {
        if i+1 < total_lines {
            add_new_numbers(&mut numbers, line);
        } else {
            operators = line.split(' ')
                .filter(|&elem| !elem.trim().is_empty())
                .collect();
        }
    }
    numbers = delete_trailing_zeroes(numbers);
    numbers = numbers.iter().copied().rev().collect::<Vec<u64>>();
    apply_operations(numbers, operators)
}

fn add_new_numbers(numbers: &mut Vec<u64>, line: &str) {
    let vec = line.chars().rev().map(|c| c.to_digit(10).unwrap_or(0) as u64).collect::<Vec<u64>>();
    if numbers.is_empty() {
        numbers.append(&mut vec.clone());
    }else {
        vec.iter().enumerate().for_each(|(i,x)| {
            numbers[i] = numbers[i] * 10 + x;
        });
    }
}

fn delete_trailing_zeroes(numbers: Vec<u64>) -> Vec<u64> {
    numbers.iter()
        .map(|&x| {let mut y = x;
            while y!= 0 && y % 10 == 0 {
                y /= 10;
            };
            y})
        .collect::<Vec<u64>>()
}

fn apply_operations(mut numbers: Vec<u64>, mut operators: Vec<&str>) -> u64 {
    let mut total = 0;
    while let Some(operator) = operators.pop() {
        let mut part = if operator == "+" {0} else {1};
        while let Some(operand) = numbers.pop() {
            if operand == 0 {
                break;
            } else {
                if operator == "+" {
                    part += operand;
                } else {
                    part *= operand;
                }
            }
        }
        total += part;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "
            .to_string();
        let result = do_homework(input);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_2() {
        let string = format!("{}{}{}{}",
                             "123 328  51 64 \n",
                             " 45 64  387 23 \n",
                             "  6 98  215 314\n",
                             "*   +   *   +  ");

        assert_eq!(do_homework(string), 3263827);
    }
}