use std::cmp::Ordering;
use std::fs::File;
use std::io::Read;
use crate::Direction::{Down, Left, Right, Up};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const DIRECTIONS: [Direction; 4] = [Up, Down, Left, Right];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Crucible {
    heat_loss: u32,
    x: usize,
    y: usize,
    las_dir: Direction,
    consecutive_moves: u8,
}

impl PartialOrd<Self> for Crucible {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.heat_loss.partial_cmp(&other.heat_loss)
    }
}

impl Ord for Crucible {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heat_loss.cmp(&other.heat_loss)
    }
}


fn main() {
    let heat_loss = minimize_heat_loss(read_input_file());
    println!("the number of energized tiles is {}", heat_loss);
}

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}

fn minimize_heat_loss(input: String) -> u32 {
    let mut matrix = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap());
        }
        matrix.push(row);
    }
    let matrix = matrix;
    let origin = Crucible {
        heat_loss:0,
        x: 0,
        y: 0,
        las_dir: Up,
        consecutive_moves: 0,
    };
    let mut stack = Vec::new();
    let mut hystory = Vec::new();
    stack.push(origin);
    while !stack.is_empty() {
        stack.sort();
        stack.reverse();
        let actual = stack.pop().unwrap();
        if actual.y == matrix.len() - 1 && actual.x == matrix[0].len() - 1 {
            return actual.heat_loss;
        }
        hystory.push((actual.x, actual.y));
        for dir in DIRECTIONS {
            if !is_opposite_direction(actual.las_dir, dir) {
                if actual.las_dir == dir && actual.consecutive_moves < 3 {
                    if let Some(mut new) = move_to(actual, dir, matrix[0].len(), matrix.len()){
                        if !hystory.iter().any(|&(x,y)| new.x == x && new.y == y) {
                            new.consecutive_moves += actual.consecutive_moves;
                            new.heat_loss += matrix[new.y][new.x];
                            stack.push(new);
                        }
                    }
                }else if actual.las_dir != dir {
                    if let Some(mut new) = move_to(actual, dir, matrix[0].len(), matrix.len()){
                        if !hystory.iter().any(|&(x,y)| new.x == x && new.y == y) {
                            new.heat_loss += matrix[new.y][new.x];
                            stack.push(new);
                        }
                    }
                }
            }
        }
    }
    0
}

fn move_to(actual: Crucible, dir: Direction, max_x: usize, max_y: usize) -> Option<Crucible> {
    let (inc_x, inc_y) = dir_to_coords(dir);
    let new_x = actual.x as i32 + inc_x;
    let new_y = actual.y as i32 + inc_y;
    if new_x >= 0 && new_x < max_x as i32 && new_y >= 0 && new_y < max_y as i32 {
        Some(Crucible {
            heat_loss: actual.heat_loss,
            x: new_x as usize,
            y: new_y as usize,
            las_dir: dir,
            consecutive_moves: 1,
        })
    } else {
        None
    }
}

fn dir_to_coords(dir: Direction) -> (i32, i32) {
    match dir {
        Up => (0, -1),
        Down => (0, 1),
        Left => (-1, 0),
        Right => (1, 0)
    }
}

fn is_opposite_direction(dir1: Direction, dir2: Direction) -> bool {
    (dir1 == Up && dir2 == Down) || (dir2 == Up && dir1 == Down) || (dir1 == Left && dir2 == Right) || (dir2 == Left && dir1 == Right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
            .to_string();
        let result = minimize_heat_loss(input);
        assert_eq!(result, 102);
    }
}