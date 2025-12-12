use std::cmp::max;

pub fn count_fresh_ingredients(input: String) -> u64 {
    let mut parsing_fresh = true;
    let mut fresh_ingredients = vec![];
    for line in input.lines() {
        if line.trim().len() == 0 {
            parsing_fresh = false;
            fresh_ingredients = unify_ranges(fresh_ingredients);
        } else if parsing_fresh {
            fresh_ingredients = add_new_fresh(fresh_ingredients, line)
        }
    }
    fresh_ingredients.iter().fold(0, |acc, (min, max)| {acc + max - min + 1})
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

fn add_new_fresh(actual_fresh: Vec<(u64, u64)>, line: &str) -> Vec<(u64, u64)> {
    let limits = line.split('-').into_iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let mut new_fresh: Vec<(u64,u64)> = actual_fresh.clone();
    new_fresh.push((limits[0].min(limits[1]), limits[1].max(limits[0])));
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
        let result = count_fresh_ingredients(input);
        assert_eq!(result, 14);
    }
}