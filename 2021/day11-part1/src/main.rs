use std::collections::HashMap;
use std::fs::{File};
use std::io::{Read};

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");
    println!("Total flashes: {}", calculate_total_flashes(contents));
}

fn calculate_total_flashes(contents: String) -> u32 {
    let mut state = parse_initial_state(contents);
    let mut total_flashes = 0;
    let steps = 100;
    for _ in 0..steps {
        let mut flashed_cells_table = create_flashed_cells_table(state.len(), state[0].len());
        simulate_step(&mut state, &mut flashed_cells_table);
        total_flashes += flashed_cells_table.iter()
                .fold(0, |acc, x|
                        acc + x.iter().filter(|y| **y).map(|_| 1).sum::<i32>());
    }
    total_flashes as u32
}

fn simulate_step(state: &mut Vec<Vec<u8>>, flashed_cells: &mut Vec<Vec<bool>>) {
    for row in 0..state.len() {
        for column in 0..state[row].len() {
            state[row][column] = (state[row][column] + 1) % 10;
        }
    }
    for row in 0..state.len() {
        for column in 0..state[row].len() {
            check_and_flash_cell(row, column, state, flashed_cells);
        }
    }
}

fn check_and_flash_cell(row: usize, column: usize, state: &mut Vec<Vec<u8>>, flashed_cells: &mut Vec<Vec<bool>>) {
    if !flashed_cells[row][column] && state[row][column] == 0 {
        flashed_cells[row][column] = true;
        let min_row = if row == 0 { 0 } else { row - 1 };
        let max_row = if row == state.len() - 1 { row } else { row + 1 };
        let min_column = if column == 0 { 0 } else { column - 1 };
        let max_column = if column == (state[row].len() - 1) { column } else { column + 1 };
        for i in min_row..max_row + 1 {
            for j in min_column..max_column + 1 {
                flash_cell(i, j, state, flashed_cells);
            }
        }
    }
}

fn flash_cell(row: usize, column: usize, state: &mut Vec<Vec<u8>>, flashed_cells: &mut Vec<Vec<bool>>) {
    if !flashed_cells[row][column] && state[row][column] != 0 {
        state[row][column] = (state[row][column] + 1) % 10;
        check_and_flash_cell(row, column, state, flashed_cells);
    }
}

fn create_flashed_cells_table(rows: usize, columns: usize) -> Vec<Vec<bool>> {
    let mut table = Vec::new();
    for _ in 0..rows {
        let mut row = Vec::new();
        for _ in 0..columns {
            row.push(false);
        }
        table.push(row);
    }
    table
}

fn parse_initial_state(table_str: String) -> Vec<Vec<u8>> {
    let mut table = Vec::new();
    for line in table_str.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as u8);
        }
        table.push(row);
    }
    table
}

#[test]
fn test_case_day11_1() {
    let points = calculate_total_flashes(String::from(
        "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"));
    assert_eq!(points, 1656)
}