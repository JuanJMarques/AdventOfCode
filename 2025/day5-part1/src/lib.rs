
pub fn count_fresh_available_ingredients(input: String) -> u32 {
    let mut parsing_fresh = true;
    let mut fresh_ingredients = vec![];
    let mut number_of_fresh_available = 0;
    for line in input.lines() {
        if line.trim().len() == 0 {
            parsing_fresh = false;
            fresh_ingredients = unify_ranges(fresh_ingredients);
        } else {
            if parsing_fresh {
                fresh_ingredients = add_new_fresh(fresh_ingredients, line)
            } else {
                let available_ingredient = line.parse::<u64>().unwrap();
                number_of_fresh_available +=
                    fresh_ingredients.iter()
                        .filter(|(min,max)| {
                            return *min <= available_ingredient && *max >= available_ingredient;
                        })
                        .count()
                        .min(1) as u32;
            }
        }
    }
    number_of_fresh_available
}

fn unify_ranges(fresh_ingredients: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut ingredients = vec![];
    ingredients.push(fresh_ingredients[0]);
    for i in 1..fresh_ingredients.len() {
        let j = ingredients.len() - 1;
        if (fresh_ingredients[i].0 <= ingredients[j].0 && fresh_ingredients[i].1 >= ingredients[j].0)
            || (fresh_ingredients[i].0 <= ingredients[j].1 && fresh_ingredients[i].1 >= ingredients[j].1) {
            ingredients[j] = (ingredients[j].0.min(fresh_ingredients[i].0), ingredients[j].1.max(fresh_ingredients[i].1));
        } else {
        ingredients.push(fresh_ingredients[i]);
        }
    }
    ingredients
}

fn add_new_fresh(actual_fresh: Vec<(u64, u64)>, line: &str) -> Vec<(u64, u64)> {
    let limits = line.split('-').into_iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let mut new_fresh: Vec<(u64,u64)> = actual_fresh.clone();
    for i in 0..new_fresh.len() {
        let mut  range = new_fresh[i];
        if range.0 < limits[0] {
            if range.1 >= limits[0] {
                // (--0--1--)
                range.1 = range.1.max(limits[1]);
                new_fresh[i] = range;
                return new_fresh;
            }
        }else if range.0 <= limits[1] && range.1 >= limits[1] {
            range.0 = range.0.min(limits[0]);
            new_fresh[i] = range;
            return new_fresh;
        } else {
            new_fresh.insert(i, (limits[0], limits[1]));
            return new_fresh;
        }
    }
    new_fresh.push((limits[0], limits[1]));
    new_fresh
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
            .to_string();
        let result = count_fresh_available_ingredients(input);
        assert_eq!(result, 3);
    }
}