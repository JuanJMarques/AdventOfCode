use std::cmp::{max, min};
use std::fs::File;
use std::io::Read;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let shortest_paths = sum_shortest_paths(read_input_file(), 1000000);
    println!("the sum of shortest paths is {}", shortest_paths);
}

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}

fn sum_shortest_paths(input: String, cost: u64) -> u64 {
    let (galaxies_map, costs) = parse_input(input);
    let (galaxies_map, costs) = expand_empty_rows(galaxies_map, costs, cost);
    let (galaxies_map, costs) = expand_empty_columns(galaxies_map, costs, cost);
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
            let costs = costs.clone();
            let sender = sender.clone();
            threads.push(thread::spawn(move || {
                sender.send(find_shortest_path(origin, destiny, max_x, max_y, costs) as u64).unwrap();
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

fn find_shortest_path(origin: (usize, usize), destiny: (usize, usize), max_x: usize, max_y: usize, costs: Vec<Vec<u64>>) -> u64 {
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
        let successors = get_next_candidates(next, max_x, max_y, costs.clone());
        let mut successors = successors.iter()
            .filter(|&&(x, y, c)| !open.iter().any(|&(nx, ny, nc)| x == nx && y==ny && nc <= c))
            .filter(|&&(x, y, c)| !open.iter().any(|&(nx, ny, nc)| x == nx && y==ny && nc <= c))
            .filter(|&&(x,y,_)| distance(x,y,destiny_x,destiny_y)<=distance(next_x,next_y,destiny_x,destiny_y))
            .copied()
            .collect::<Vec<(usize, usize, u64)>>();
        open.append(&mut successors);
    }
    0
}

fn distance(x: usize, y: usize, dx: usize, dy: usize) -> f32 {
    (((max(x,dx) - min(x,dx)).pow(2) + (max(y,dy) - min(y,dy)).pow(2)) as f32).sqrt()
}

fn get_next_candidates(previous: (usize, usize, u64), max_x: usize, max_y: usize, costs: Vec<Vec<u64>>) -> Vec<(usize, usize, u64)> {
    let (x,y,c) = previous;
    let mut candidates = Vec::new();
    if x > 0 {
        candidates.push((x - 1, y, c + costs[x-1][y]));
    }
    if x+1 < max_x {
        candidates.push((x + 1, y, c + costs[x+1][y]));
    }
    if y > 0 {
        candidates.push((x , y-1, c + costs[x][y-1]));
    }
    if y+1 < max_y {
        candidates.push((x, y+1, c + costs[x][y+1]));
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

fn expand_empty_columns(galaxies_map: Vec<Vec<char>>, costs: Vec<Vec<u64>>, cost: u64) -> (Vec<Vec<char>>, Vec<Vec<u64>>) {
    let galaxies_map = transpose(galaxies_map);
    let costs = transpose(costs);
    let (galaxies_map, costs) = expand_empty_rows(galaxies_map, costs, cost);
    (transpose(galaxies_map),transpose(costs))
}

fn transpose<T: Clone + Copy>(galaxies_map: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut new_galaxies = Vec::new();
    for j in 0..galaxies_map[0].len() {
        let mut new_row = Vec::new();
        for row in galaxies_map.clone() {
            new_row.push(row[j]);
        }
        new_galaxies.push(new_row);
    }
    new_galaxies.clone()
}

fn expand_empty_rows(galaxies_map: Vec<Vec<char>>, mut costs: Vec<Vec<u64>>, cost: u64) -> (Vec<Vec<char>>, Vec<Vec<u64>>) {
    for (i, row) in galaxies_map.iter().enumerate() {
        if row.iter().all(|&c| c == '.') {
            costs[i] = vec![cost; costs[0].len()];
        }
    }
    (galaxies_map,costs)
}

fn parse_input(input: String) -> (Vec<Vec<char>>, Vec<Vec<u64>>) {
    let vec = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let costs = input.lines().map(|line| vec![1; line.len()]).collect::<Vec<Vec<u64>>>();
    (vec,costs)
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
        let result = sum_shortest_paths(input,10);
        assert_eq!(result, 1030);
    }

    #[test]
    fn test_input_2() {
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
        let result = sum_shortest_paths(input,100);
        assert_eq!(result, 8410);
    }
}