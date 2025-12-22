pub fn count_beam_splits(input: String) -> u64 {
    let (start_point,splitter_matrix) = parse_input(input);
    let mut visited = vec![vec![0; splitter_matrix[0].len()]; splitter_matrix.len()];
    1 + follow_bean(start_point,splitter_matrix, &mut visited)
}
fn parse_input(input: String) -> ((i32, i32), Vec<Vec<bool>>) {
    let mut start_point = (0_i32, 0_i32);
    for (i,line) in input.lines().enumerate() {
        if line.contains('S') {
            start_point = (i as i32,line.find('S').unwrap() as i32);
            break;
        }
    }
    let mut splitter_matrix = input.lines()
        .map(|line| {line.chars().map(|c| if c == '^' {true} else {false})
            .collect::<Vec<bool>>()})
        .collect::<Vec<Vec<bool>>>();
    (start_point,splitter_matrix)
}

fn follow_bean(point: (i32, i32), splitter_matrix: Vec<Vec<bool>>, visited: &mut Vec<Vec<u64>>) -> u64 {
    if point.0 < 0 || point.1 <= 0
        || point.1 as usize >= splitter_matrix[0].len() {
        return 0;
    }
    if point.0 as usize >= splitter_matrix.len() {
        return 1;
    }
    if visited[point.0 as usize][point.1 as usize] != 0 {
        return visited[point.0 as usize][point.1 as usize];
    }
    let is_splitter = splitter_matrix[point.0 as usize][point.1 as usize];
    if is_splitter {
        visited[point.0 as usize][point.1 as usize] =
             follow_bean((point.0, point.1 - 1),splitter_matrix.clone(), visited)
            + follow_bean((point.0, point.1 + 1),splitter_matrix, visited);
    } else {
        visited[point.0 as usize][point.1 as usize] = follow_bean((point.0 + 1, point.1),splitter_matrix, visited);
    }
    visited[point.0 as usize][point.1 as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
            .to_string();
        let result = count_beam_splits(input);
        assert_eq!(result, 40);
    }
}