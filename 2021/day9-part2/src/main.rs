use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");
    let mut points = vec![];
    for line in contents.lines() {
        points.push(line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>());
    }
    let mut low_points_coords = vec![];
    for i in 0..points.len() {
        for j in 0..points[i].len() {
            let test_point = points[i][j];
            if check_lowest_point(points.clone(), i, j, test_point) {
                println!("point {}, at {},{} is a low point", test_point, i, j);
                low_points_coords.push((test_point, i, j));
            }
        }
    }
    let mut basin_sizes = vec![];
    for coord in low_points_coords {
        basin_sizes.push(get_basin_size(points.clone(), coord.1, coord.2, coord.0, vec![].as_mut()));
    }
    println!("basin sizes {:?}", basin_sizes);
    basin_sizes.sort();
    basin_sizes.reverse();
    println!("basin sizes sorted {:?}", basin_sizes);
    println!("three largest basin sizes multiplied: {}", basin_sizes[0] * basin_sizes[1] * basin_sizes[2]);
}

fn get_basin_size(points: Vec<Vec<u8>>, i: usize, j: usize, value: u8, visited_cells: &mut Vec<(usize, usize)>) -> u32 {
    if visited_cells.contains(&(i, j)) {
        return 0;
    }
    visited_cells.push((i, j));
    let mut size = 1;
    if i > 0 && points[i - 1][j] > value && points[i - 1][j] != 9 {
        size += get_basin_size(points.clone(), i - 1, j, points[i - 1][j], visited_cells);
    }
    if i + 1 < points.len() && points[i + 1][j] > value && points[i + 1][j] != 9 {
        size += get_basin_size(points.clone(), i + 1, j, points[i + 1][j], visited_cells);
    }
    if j > 0 && points[i][j - 1] > value && points[i][j - 1] != 9 {
        size += get_basin_size(points.clone(), i, j - 1, points[i][j - 1], visited_cells);
    }
    if j + 1 < points[i].len() && points[i][j + 1] > value && points[i][j + 1] != 9 {
        size += get_basin_size(points.clone(), i, j + 1, points[i][j + 1], visited_cells);
    }
    size
}

fn check_lowest_point(points: Vec<Vec<u8>>, row: usize, column: usize, test_point: u8) -> bool {
    let up = match row.checked_sub(1) {
        Some(x) => points[x][column],
        None => test_point + 1,
    };
    let down = match row + 1 < points.len() {
        true => points[row + 1][column],
        false => test_point + 1,
    };
    let left = match column.checked_sub(1) {
        Some(x) => points[row][x],
        None => test_point + 1,
    };
    let right = match column + 1 < points[row].len() {
        true => points[row][column + 1],
        false => test_point + 1,
    };
    test_point < up && test_point < down && test_point < left && test_point < right
}

