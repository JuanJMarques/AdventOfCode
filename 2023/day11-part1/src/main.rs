use std::cmp::{max, min};
use std::fs::File;
use std::io::Read;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let shortest_paths = sum_shortest_paths(read_input_file());
    println!("the sum of shortest paths is {}", shortest_paths);
}

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}

fn sum_shortest_paths(input: String) -> u64 {
    let galaxies_map = parse_input(input);
    let galaxies_map = expand_empty_rows(galaxies_map);
    let galaxies_map = expand_empty_columns(galaxies_map);
    let galaxies = find_galaxies_coords(galaxies_map.clone());
    let max_x = galaxies_map.len();
    let max_y = galaxies_map[0].len();
    let (sender, receiver) = channel();
    let mut threads = Vec::new();
    let mut count = 0;
    for i in 0..galaxies.len() - 1 {
        for j in i+1..galaxies.len() {
            let origin = galaxies[i];
            let destiny = galaxies[j];
            let sender = sender.clone();
            threads.push(thread::spawn(move || {
                sender.send(find_shortest_path(origin, destiny, max_x, max_y) as u64).unwrap();
            }));
            count+=1;
        }
    }

    for thread in threads {
        thread.join().unwrap();
    }
    let mut sum = 0;
    for _ in 0..count {
        sum += receiver.recv().unwrap();
    }
    sum
}

fn find_shortest_path(origin: (usize, usize), destiny: (usize, usize), max_x: usize, max_y: usize) -> u32 {
    let mut open = Vec::new();
    let mut closed = Vec::new();
    let (origin_x, origin_y) = origin;
    open.push((origin_x, origin_y, 0));
    let (destiny_x, destiny_y) = destiny;
    while !open.is_empty() {
        open.sort_by(|&(_, _, c1), (_, _, c2)| c1.cmp(c2));
        open.reverse();
        let next = open.pop().unwrap();
        let (next_x, next_y, cost) = next;
        closed.push(next);
        if next_x == destiny_x && next_y == destiny_y {
            return cost;
        }
        let successors = get_next_candidates(next, max_x, max_y);
        let mut successors = successors.iter()
            .filter(|&&(x, y, c)| !open.iter().any(|&(nx, ny, nc)| x == nx && y==ny && nc <= c))
            .filter(|&&(x, y, c)| !open.iter().any(|&(nx, ny, nc)| x == nx && y==ny && nc <= c))
            .filter(|&&(x,y,_)| distance(x,y,destiny_x,destiny_y)<=distance(next_x,next_y,destiny_x,destiny_y))
            .copied()
            .collect::<Vec<(usize, usize, u32)>>();
        open.append(&mut successors);
        // for elem in successors {
        //     let (x,y,c) = elem;
        //     if !open.iter().any(|&(nx, ny, nc)| x == nx && y==ny && nc <= c) && !closed.iter().any(|&(nx, ny, nc)| x == nx && y==ny && nc <= c) {
        //         open.push(elem);
        //     }
        // }
    }
    0
}

fn distance(x: usize, y: usize, dx: usize, dy: usize) -> f32 {
    (((max(x,dx) - min(x,dx)).pow(2) + (max(y,dy) - min(y,dy)).pow(2)) as f32).sqrt()
}

fn get_next_candidates(previous: (usize, usize, u32), max_x: usize, max_y: usize) -> Vec<(usize, usize, u32)> {
    let (x,y,c) = previous;
    let mut candidates = Vec::new();
    if x > 0 {
        candidates.push((x - 1, y, c + 1));
    }
    if x+1 < max_x {
        candidates.push((x + 1, y, c + 1));
    }
    if y > 0 {
        candidates.push((x , y-1, c + 1));
    }
    if y+1 < max_y {
        candidates.push((x, y+1, c + 1));
    }
    candidates
}

fn find_galaxies_coords(galaxies_map: Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();
    for (i, row) in galaxies_map.iter().enumerate() {
        for (j, &candidate) in row.iter().enumerate() {
            if candidate == '#' {
                galaxies.push((i, j));
            }
        }
    }
    galaxies
}

fn expand_empty_columns(galaxies_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let galaxies_map = transpose(galaxies_map);
    let galaxies_map = expand_empty_rows(galaxies_map);
    transpose(galaxies_map)
}

fn transpose(galaxies_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_galaxies = Vec::new();
    for j in 0..galaxies_map[0].len() {
        let mut new_row = Vec::new();
        for row in galaxies_map.clone() {
            new_row.push(row[j]);
        }
        new_galaxies.push(new_row);
    }
    new_galaxies
}

fn expand_empty_rows(galaxies_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_galaxies = Vec::new();
    for row in galaxies_map {
        if row.iter().all(|&c| c == '.') {
            new_galaxies.push(row.clone());
        }
        new_galaxies.push(row.clone());
    }
    new_galaxies
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
            .to_string();
        let result = sum_shortest_paths(input);
        assert_eq!(result, 374);
    }
}