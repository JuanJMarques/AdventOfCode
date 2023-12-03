use std::fs::File;
use std::io::Read;

fn main() {
    let sum_possible_games = sum_games_power(read_input_file());
    println!("The sum of possible games is {}", sum_possible_games)
}

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}

fn sum_games_power(input: String) -> u32 {
    let mut sum_of_games_power = 0;
    for line in input.lines() {
        let mut red_power = 1;
        let mut green_power = 1;
        let mut blue_power = 1;
        let game = line.split(':').collect::<Vec<&str>>();
        let subsets = game[1].split(';').collect::<Vec<&str>>();
        subsets.iter().for_each(|subset| {
            (
                red_power, green_power, blue_power) = get_new_game_powers(subset, red_power, green_power, blue_power);
        });
        sum_of_games_power += red_power * green_power * blue_power;
    }
    sum_of_games_power
}

fn get_new_game_powers(subset: &str, red_power: u32, green_power: u32, blue_power: u32) -> (u32, u32, u32) {
    let cube_sets: Vec<&str> = subset.split(',').collect();
    let mut min_red = red_power;
    let mut min_green = green_power;
    let mut min_blue = blue_power;
    for cube_set in cube_sets {
        let set_info: Vec<&str> = cube_set.trim().split(' ').collect();
        let number = set_info[0].trim().parse::<u32>().unwrap();
        let color = set_info[1].trim();
        match color {
            "red" => {
                if min_red < number {
                    min_red = number;
                }
            }
            "green" => {
                if min_green < number {
                    min_green = number;
                }
            }
            "blue" => {
                if min_blue < number {
                    min_blue = number;
                }
            }
            _ => {}
        };
    }
    (min_red, min_green, min_blue)
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
        let result = sum_games_power(input);
        assert_eq!(result, 2286);
    }
}
