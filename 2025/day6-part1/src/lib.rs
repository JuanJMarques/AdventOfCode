pub fn do_homework(input: String) -> u64 {
    let mut total = 0;
    let mut numbers = vec![];
    let mut operands = vec![];
    for line in input.lines() {
        if line.starts_with('+') || line.starts_with('*') {
            operands = line.split(' ')
                .filter(|&elem| !elem.trim().is_empty())
                .collect();
        } else {
            add_new_numbers(&mut numbers, line)
        }
    }
    for i in 0..operands.len() {
        total += apply_operand(operands[i], numbers[i].clone());
    }
    total
}

fn apply_operand(operand: &str, numbers: Vec<u64>) -> u64 {
    if operand == "+" {
        numbers.iter().sum::<u64>()
    }else {
        numbers.iter().product::<u64>()
    }
}

fn unify_ranges(fresh_ingredients: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut fresh_ingredients = fresh_ingredients.clone();
    fresh_ingredients.sort_by(|a, b| a.0.cmp(&b.0));
    let mut ingredients = vec![];
    ingredients.push(fresh_ingredients[0]);
    for i in 1..fresh_ingredients.len() {
        let j = ingredients.len() - 1;
        if (fresh_ingredients[i].0 <= ingredients[j].0 && fresh_ingredients[i].1 >= ingredients[j].1)
            || (fresh_ingredients[i].0 >= ingredients[j].0 && fresh_ingredients[i].1 <= ingredients[j].1)
            || (fresh_ingredients[i].0 <= ingredients[j].1 && fresh_ingredients[i].1 >= ingredients[j].1)
            || (fresh_ingredients[i].0 <= ingredients[j].0 && fresh_ingredients[i].1 >= ingredients[j].0){
            ingredients[j] = (ingredients[j].0.min(fresh_ingredients[i].0), ingredients[j].1.max(fresh_ingredients[i].1));
        } else {
        ingredients.push(fresh_ingredients[i]);
        }
    }
    ingredients
}

fn add_new_numbers(number_lists: &mut Vec<Vec<u64>>, line: &str) {
    let numbers = line.split(' ')
        .filter(|&elem| !elem.trim().is_empty())
        .map(|elem| elem.parse::<u64>().unwrap_or(0))
        .collect::<Vec<u64>>();
    if number_lists.len() == 0 {
        number_lists.append(&mut numbers.iter().map(|&x| vec![x]).collect::<Vec<Vec<u64>>>());
    }else {
        for i in 0..number_lists.len() {
            number_lists[i].push(numbers[i]);
        }
    }

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
}