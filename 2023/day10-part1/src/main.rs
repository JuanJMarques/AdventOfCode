use crate::Direction::{Down, Left, Right, Up};
use std::fs::File;
use std::io::Read;

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone)]
struct Animal {
    x: usize,
    y: usize,
    direction: Direction,
    distance: u32,
}

impl Animal {
    fn move_to_next_pipe(
        &mut self,
        mut path_distances: Vec<Vec<u32>>,
        pipes_map: Vec<Vec<char>>,
    ) -> Vec<Vec<u32>> {
        (self.x, self.y) = match self.direction {
            Up => (self.x, self.y - 1),
            Down => (self.x, self.y + 1),
            Left => (self.x - 1, self.y),
            Right => (self.x + 1, self.y),
        };
        self.distance += 1;
        if self.distance < path_distances[self.y][self.x] {
            path_distances[self.y][self.x] = self.distance;
        }
        self.direction = get_pipe_direction(self.x, self.y, pipes_map, self.direction);
        path_distances
    }
}

fn get_pipe_direction(
    x: usize,
    y: usize,
    pipes_map: Vec<Vec<char>>,
    prev_direction: Direction,
) -> Direction {
    let pipe = pipes_map[y][x];
    match pipe {
        '|' => match prev_direction {
            Up => Up,
            Down => Down,
            _ => {
                panic!("Impossible Direction")
            }
        },
        '-' => match prev_direction {
            Left => Left,
            Right => Right,
            _ => {
                panic!("Impossible Direction")
            }
        },
        'L' => match prev_direction {
            Down => Right,
            Left => Up,
            _ => {
                panic!("Impossible Direction")
            }
        },
        'J' => match prev_direction {
            Down => Left,
            Right => Up,
            _ => {
                panic!("Impossible Direction")
            }
        },
        '7' => match prev_direction {
            Up => Left,
            Right => Down,
            _ => {
                panic!("Impossible Direction")
            }
        },
        'F' => match prev_direction {
            Up => Right,
            Left => Down,
            _ => {
                panic!("Impossible Direction")
            }
        },
        'S' => Up,
        _ => {
            panic!("Unknown pipe {}", pipe)
        }
    }
}

fn detect_start_position(pipes_map: Vec<Vec<char>>) -> (usize, usize) {
    for (y, line) in pipes_map.iter().enumerate() {
        for (x, &char) in line.iter().enumerate() {
            if char == 'S' {
                return (x, y);
            }
        }
    }
    panic!("Start point not found")
}

fn detect_valid_starting_pipes(
    pipes_map: Vec<Vec<char>>,
    x: usize,
    y: usize,
) -> (Direction, Direction) {
    let mut first = None;
    if x > 0 {
        let left = pipes_map[y][x - 1];
        if left == '-' || left == 'L' || left == 'F' {
            if first.is_none() {
                first = Some(Left);
            } else {
                return (first.unwrap(), Left);
            }
        }
    }
    if x + 1 < pipes_map[0].len() {
        let right = pipes_map[y][x + 1];
        if right == '-' || right == 'J' || right == '7' {
            if first.is_none() {
                first = Some(Right);
            } else {
                return (first.unwrap(), Right);
            }
        }
    }
    if y > 0 {
        let up = pipes_map[y - 1][x];
        if up == '|' || up == '7' || up == 'F' {
            if first.is_none() {
                first = Some(Up);
            } else {
                return (first.unwrap(), Up);
            }
        }
    }
    if y + 1 < pipes_map.len() {
        let down = pipes_map[y + 1][x];
        if down == '|' || down == 'L' || down == 'J' {
            if first.is_none() {
                first = Some(Down);
            } else {
                return (first.unwrap(), Down);
            }
        }
    }
    panic!("not valid pipes found at starting point");
}

fn main() {
    let longest_distance = get_longest_distance(read_input_file());
    println!("the longest distance is {}", longest_distance);
}

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}

fn get_longest_distance(input: String) -> u32 {
    let (pipes_map, mut distance) = parse_input(input);
    let (start_x, start_y) = detect_start_position(pipes_map.clone());
    distance[start_y][start_x] = 0;
    let (dir1, dir2) = detect_valid_starting_pipes(pipes_map.clone(), start_x, start_y);
    let mut animal = Animal {
        x: start_x,
        y: start_y,
        direction: dir1,
        distance: 0,
    };
    let mut start = true;
    while start || pipes_map[animal.y][animal.x] != 'S' {
        start = false;
        distance = animal.move_to_next_pipe(distance.clone(), pipes_map.clone());
    }
    animal = Animal {
        x: start_x,
        y: start_y,
        direction: dir2,
        distance: 0,
    };
    let mut start = true;
    while start || pipes_map[animal.y][animal.x] != 'S' {
        start = false;
        distance = animal.move_to_next_pipe(distance.clone(), pipes_map.clone());
    }
    distance
        .iter()
        .map(|line| {
            line.iter()
                .map(|&x| if x != u32::MAX { x } else { 0 })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn parse_input(input: String) -> (Vec<Vec<char>>, Vec<Vec<u32>>) {
    let mut pipes = Vec::new();
    let mut distances = Vec::new();
    for line in input.lines() {
        let mut pipes_line = Vec::new();
        let mut distances_line = Vec::new();
        for char in line.chars() {
            pipes_line.push(char);
            distances_line.push(u32::MAX);
        }
        pipes.push(pipes_line);
        distances.push(distances_line);
    }
    (pipes, distances)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = ".....
.S-7.
.|.|.
.L-J.
....."
            .to_string();
        let result = get_longest_distance(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_input_2() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
            .to_string();
        let result = get_longest_distance(input);
        assert_eq!(result, 8);
    }
}
