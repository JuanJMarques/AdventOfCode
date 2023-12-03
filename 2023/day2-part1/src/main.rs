use std::fs::File;
use std::io::Read;

const MAX_RED_CUBES: u32 = 12;
const MAX_GREEN_CUBES: u32 = 13;
const MAX_BLUE_CUBES: u32 = 14;

fn main() {
    let sum_possible_games = sum_possible_games(read_input_file());
    println!("The sum of possible games is {}", sum_possible_games)
}

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}

fn sum_possible_games(input: String) -> u32 {
    let mut sum_of_possible_games = 0;
    for line in input.lines() {
        let game = line.split(':').collect::<Vec<&str>>();
        let game_id = get_game_id(game[0].trim());
        let subsets = game[1].split(';').collect::<Vec<&str>>();
        if subsets.iter().all(|subset| is_possible_subset(subset)) {
            sum_of_possible_games += game_id;
        }
    }
    sum_of_possible_games
}

fn is_possible_subset(subset: &str) -> bool {
    let cube_sets: Vec<&str> = subset.split(',').collect();
    let mut is_possible = true;
    for cube_set in cube_sets {
        let set_info: Vec<&str> = cube_set.trim().split(' ').collect();
        let number = set_info[0].trim().parse::<u32>().unwrap();
        let color = set_info[1].trim();
        is_possible = is_possible && number <= match color {
            "red" => MAX_RED_CUBES,
            "green" => MAX_GREEN_CUBES,
            "blue" => MAX_BLUE_CUBES,
            _ => 0
        };
    }
    is_possible
}

fn get_game_id(game_index: &str) -> u32 {
    game_index.split(' ')
        .collect::<Vec<&str>>()[1]
        .trim().parse::<u32>()
        .unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input =
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
             Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
             Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
             Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
             Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string();
        let result = sum_possible_games(input);
        assert_eq!(result, 8);
    }
}
