use std::fs::File;
use std::io::Read;
use colour::{e_green, e_red};

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let mut read_board = false;
    let mut numbers: Vec<i32> = Vec::new();
    let mut boards: Vec<Vec<Vec<(i32, bool)>>> = Vec::new();
    let mut board_index: usize = 0;
    for mut line in contents.lines() {
        if line.trim().is_empty() {
            if read_board {
                board_index += 1;
            } else {
                read_board = true;
            }
            boards.push(Vec::new());
        } else {
            if read_board {
                let row = line.trim()
                        .split_whitespace()
                        .map(|x| (x.parse::<i32>().unwrap(), false))
                        .collect::<Vec<(i32, bool)>>();
                boards[board_index].push(row);
            } else {
                numbers = line.trim()
                        .split(",")
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
            }
        }
    }
    let mut winning_board_index = 0;
    let mut number_index = 0;
    let mut last_number = 0;
    let mut active_boards = vec![true; boards.len()];
    while active_boards.iter().any(|x| *x) && number_index < numbers.len() {
        let actual_number = numbers[number_index];
        last_number = actual_number;
        for (board_index, board) in boards.iter_mut().enumerate() {
            if active_boards[board_index] {
                mark_number(board, actual_number);
                if check_board(board) {
                    active_boards[board_index] = false;
                    winning_board_index = board_index;
                }
            }
        }
        if active_boards.iter().any(|x| *x) {
            number_index += 1;
        }
    }
    if active_boards.iter().any(|x| *x) {
        panic!("All numbers shown but still active boards {:?}", active_boards);
    }
    println!("Last number: {}", last_number);
    println!("Last winning board: ");
    let mut unmarked_cells_value = 0;
    for row in boards[winning_board_index].clone() {
        for cell in row.clone() {
            if cell.1 {
                e_green!("{:>2} ", cell.0);
            } else {
                unmarked_cells_value += cell.0;
                e_red!("{:>2} ", cell.0);
            }
        }
        println!();
    }
    let score = unmarked_cells_value * last_number;
    println!("Score: {}", score);
}

fn check_board(board: &Vec<Vec<(i32, bool)>>) -> bool {
    //horizontal check
    if horizontal_check(board) {
        return true;
    }
    let transposed_board = transpose(board);
    horizontal_check(&transposed_board)
}

fn transpose(board: &Vec<Vec<(i32, bool)>>) -> Vec<Vec<(i32, bool)>> {
    let mut transposed_board = Vec::new();
    transposed_board = vec![vec![(0, false); board[0].len()]; board.len()];
    for (row_index, row) in board.iter().enumerate() {
        for (column_index, (number, checked)) in row.iter().enumerate() {
            transposed_board[column_index][row_index] = (*number, *checked);
        }
    }
    transposed_board
}

fn horizontal_check(board: &Vec<Vec<(i32, bool)>>) -> bool {
    board.iter()
            .filter(|row|
                    row.iter().map(|x| x.1).all(|x| x)
            )
            .count() > 0
}

fn mark_number(board: &mut Vec<Vec<(i32, bool)>>, number: i32) {
    for row in board.iter_mut() {
        for cell in row.iter_mut() {
            if cell.0 == number {
                cell.1 = true;
            }
        }
    }
}
