use std::fs::File;
use std::io::Read;

fn main() {
    let total_arrangements = get_total_arrangements(read_input_file());
    println!("the sum of arrangements is {}", total_arrangements);
}

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}

fn get_total_arrangements(input: String) -> u32 {
    input.lines()
        .map(count_number_of_arrangements)
        .sum()
}

fn count_number_of_arrangements(line: &str) -> u32{
    let parts = line.split(' ').collect::<Vec<&str>>();
    let conditions = parts[0].trim();
    let groups = parse_groups(parts[1].trim());
    if let (groups_count, true) = count_arrangements(conditions, groups){
        return groups_count
    }
    0
}

fn count_arrangements_point(conditions: &str, groups: Vec<u32>) -> (u32, bool) {
    if conditions.is_empty() {
        return (1, groups.is_empty());
    }
    match conditions.chars().next().unwrap() {
        '.' => {
            count_arrangements(&conditions[1..conditions.len()], groups)
        },
        '#' => {
            (0,false)
        }
        '?' => {
            count_arrangements(&conditions[1..conditions.len()], groups)
        }
        x => panic!("unknown character {}", x),
    }
}

fn count_arrangements_sharp(conditions: &str, groups: Vec<u32>) -> (u32, bool) {
    if conditions.is_empty() {
        return (1, groups.is_empty());
    }
    match conditions.chars().next().unwrap() {
        '.' => {
            (0,false)
        },
        '#' => {
            if groups.is_empty() {
                return (0, false);
            }
            let mut groups = groups.clone();
            let mut count = groups.remove(0);
            count -= 1;
            if count == 0 {
                return count_arrangements_point(&conditions[1..conditions.len()], groups)
            } else {
                groups.insert(0, count);
            }
            count_arrangements_sharp(&conditions[1..conditions.len()], groups)
        }
        '?' => {
            if groups.is_empty() {
                return (0, false);
            }
            let mut groups = groups.clone();
            let mut count = groups.remove(0);
            count -= 1;
            if count == 0 {
                return count_arrangements_point(&conditions[1..conditions.len()], groups)
            } else {
                groups.insert(0, count);
            }
            count_arrangements_sharp(&conditions[1..conditions.len()], groups)
        }
        x => panic!("unknown character {}", x),
    }
}

fn count_arrangements(conditions: &str, groups: Vec<u32>) -> (u32, bool) {
    if conditions.is_empty() {
        return (1, groups.is_empty());
    }
    match conditions.chars().next().unwrap() {
        '.' => {
            count_arrangements(&conditions[1..conditions.len()], groups)
        },
        '#' => {
            if groups.is_empty() {
                return (0, false);
            }
            let mut groups = groups.clone();
            let mut count = groups.remove(0);
            count -= 1;
            if count == 0 {
                return count_arrangements_point(&conditions[1..conditions.len()], groups)
            } else {
                groups.insert(0, count);
            }
            count_arrangements_sharp(&conditions[1..conditions.len()], groups)
        }
        '?' => {
            if groups.is_empty() {
                return count_arrangements(&conditions[1..conditions.len()], groups)
            }
            let (mut variants, success) = count_arrangements(&conditions[1..conditions.len()], groups.clone());
            if !success {
                variants = 0;
            }
            let mut groups = groups.clone();
            let mut count = groups.remove(0);
            count -= 1;
            if count == 0 {
                let (aux, success) = count_arrangements_point(&conditions[1..conditions.len()], groups);
                if success {
                    variants += aux;
                }
            } else {
                groups.insert(0, count);
                let (aux, success) = count_arrangements_sharp(&conditions[1..conditions.len()], groups);
                if success {
                    variants += aux;
                }
            }
            (variants, variants>0)
        }
        x => panic!("unknown character {}", x),
    }
}

fn parse_groups(groups_str: &str) -> Vec<u32> {
    groups_str.split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = ".???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
//         let input = ".??..??...?##. 1,1,3"
            .to_string();
        let result = get_total_arrangements(input);
        assert_eq!(result, 21);
    }
}