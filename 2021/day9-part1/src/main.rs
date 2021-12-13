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
    let mut low_poins = vec![];
    for i in 0..points.len() {
        for j in 0..points[i].len() {
            let test_point = points[i][j];
            if check_lowest_point(points.clone(), i, j, test_point) {
                println!("point {}, at {},{} is a low point", test_point, i, j);
                low_poins.push(test_point);
            }
        }
    }
    let low_points_risk_level = low_poins.iter().map(|x| (x + 1) as u32).sum::<u32>();
    println!("low points risk level is {}", low_points_risk_level);
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

