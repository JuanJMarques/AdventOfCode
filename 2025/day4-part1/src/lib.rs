
pub fn find_accessible_rolls(input: String) -> u32 {
    let matrix = parse_matrix(input);
    count_accessible_rolls(matrix)
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

fn count_accessible_rolls(matrix: Vec<Vec<u8>>) -> u32 {
    let mut accessible_rolls: u32 = 0;
    for (i,row) in matrix.iter().enumerate() {
        for (j,&n) in row.iter().enumerate() {
            if n != 0 {
                let mini = i.max(1) - 1;
                let maxi = (i+1).min(matrix.len() -1);
                let minj = j.max(1) - 1;
                let maxj = (j+1).min(matrix[0].len() -1);
                let mut  roll_count = 0;
                for x in mini..=maxi {
                    for y in minj..=maxj {
                        if !(x == i && y == j) {
                            roll_count += matrix[x][y];
                        }
                    }
                }
                if roll_count < 4 {
                    accessible_rolls += 1;
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
    accessible_rolls
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
        let result = find_accessible_rolls(input);
        assert_eq!(result, 13);
    }
}