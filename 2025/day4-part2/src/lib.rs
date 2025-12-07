
pub fn remove_rolls(input: String) -> u32 {
    let matrix = parse_matrix(input);
    remove_accessible_rolls(matrix)
}

fn parse_matrix(input: String) -> Vec<Vec<u8>> {
    input.lines()
        .map(|line| {
            line
                .chars()
                .enumerate()
                .map(|(_, c)| if c == '@' { 1 } else { 0 })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>()
}

fn remove_accessible_rolls(matrix: Vec<Vec<u8>>) -> u32 {
    let mut matrix = matrix.clone();
    let mut removed_rolls: u32 = 0;
    let mut any_roll_removed = true;
    while any_roll_removed {
        any_roll_removed = false;
        let mut new_matrix = matrix.clone();
        for (i, row) in matrix.clone().iter().enumerate() {
            for (j, &n) in row.iter().enumerate() {
                if n != 0 {
                    let mini = i.max(1) - 1;
                    let maxi = (i + 1).min(matrix.len() - 1);
                    let minj = j.max(1) - 1;
                    let maxj = (j + 1).min(matrix[0].len() - 1);
                    let mut roll_count = 0;
                    for x in mini..=maxi {
                        for y in minj..=maxj {
                            if !(x == i && y == j) {
                                roll_count += matrix[x][y];
                            }
                        }
                    }
                    if roll_count < 4 {
                        any_roll_removed = true;
                        removed_rolls += 1;
                        new_matrix[i][j] = 0;
                        print!("x")
                    } else {
                        print!("@")
                    }
                } else {
                    print!(".")
                }
            }
            println!();
        }
        matrix = new_matrix.clone();
        println!();
    }
    removed_rolls
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            .to_string();
        let result = remove_rolls(input);
        assert_eq!(result, 43);
    }
}