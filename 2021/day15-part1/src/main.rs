use std::cmp::Ordering;
use std::fs::{File};
use std::hash::{Hash, Hasher};
use std::io::{Read};
use colour::{green};


fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");
    let risk = calculate_lowest_path_risk(contents);
    println!("{}", risk);
}

#[derive(Clone, Debug)]
struct PathNode {
    cord: (usize, usize),
    cost: u32,
    distance: u32,
    risk: u8,
    total_cost: u32,
    history: Vec<(usize, usize)>,
}

impl Eq for PathNode {}

impl PartialEq<Self> for PathNode {
    fn eq(&self, other: &Self) -> bool {
        self.cord.0 == other.cord.0 && self.cord.1 == other.cord.1
    }
}

impl PartialOrd<Self> for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.total_cost.partial_cmp(&other.total_cost)
    }
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_cost.cmp(&other.total_cost)
    }
}

impl Hash for PathNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cord.0.hash(state);
        self.cord.1.hash(state);
    }
}

impl PathNode {
    fn new(cord: (usize, usize), cost: u32, destiny_cords: (usize, usize), risk: u8, history: Vec<(usize, usize)>) -> Self {
        let distance = euclidean_distance(cord, destiny_cords) as u32;
        PathNode {
            cord,
            cost,
            distance,
            risk,
            total_cost: cost + distance,
            history,
        }
    }
}

fn euclidean_distance(origin: (usize, usize), destiny: (usize, usize)) -> usize {
    let x_diff = destiny.0 - origin.0;
    let y_diff = destiny.1 - origin.1;
    ((x_diff.pow(2) + y_diff.pow(2)) as f32).sqrt() as usize
}

fn calculate_lowest_path_risk(content: String) -> u32 {
    let grid = parse_input(content);
    let mut candidates = vec![PathNode::new((0, 0), 0, (grid.len(), grid[0].len()), grid[0][0], Vec::new())];
    let mut destiny = PathNode::new((grid.len() - 1, grid[0].len() - 1), 0, (grid.len(), grid[0].len()), grid[grid.len() - 1][grid[0].len() - 1], Vec::new());
    let mut closed = vec![];
    let mut path_found = false;
    while !candidates.is_empty() && !path_found {
        candidates.sort();
        candidates.reverse();
        let current_node = candidates.pop().unwrap();
        if current_node == destiny {
            println!("{:?}", current_node);
            path_found = true;
            destiny = current_node;
            break;
        }
        let up = current_node.cord.0.checked_sub(1)
                .map(|x| {
                    let mut new_history = current_node.history.clone();
                    new_history.push(current_node.cord);
                    PathNode::new((x, current_node.cord.1),
                                  current_node.cost + grid[x][current_node.cord.1] as u32,
                                  (grid.len(), grid[0].len()),
                                  grid[x][current_node.cord.1],
                                  new_history)
                });
        let down = current_node.cord.0.checked_add(1)
                .filter(|&x| x < grid.len())
                .map(|x| {
                    let mut new_history = current_node.history.clone();
                    new_history.push(current_node.cord);
                    PathNode::new((x, current_node.cord.1),
                                  current_node.cost + grid[x][current_node.cord.1] as u32,
                                  (grid.len(), grid[0].len()),
                                  grid[x][current_node.cord.1],
                                  new_history)
                });

        let left = current_node.cord.1.checked_sub(1)
                .map(|y| {
                    let mut new_history = current_node.history.clone();
                    new_history.push(current_node.cord);
                    PathNode::new((current_node.cord.0, y),
                                  current_node.cost + grid[current_node.cord.0][y] as u32,
                                  (grid.len(), grid[0].len()),
                                  grid[current_node.cord.0][y],
                                  new_history)
                });

        let right = current_node.cord.1.checked_add(1)
                .filter(|&y| y < grid[0].len())
                .map(|y| {
                    let mut new_history = current_node.history.clone();
                    new_history.push(current_node.cord);
                    PathNode::new((current_node.cord.0, y),
                                  current_node.cost + grid[current_node.cord.0][y] as u32,
                                  (grid.len(), grid[0].len()),
                                  grid[current_node.cord.0][y],
                                  new_history)
                });

        let neighbours = vec![up, down, left, right];
        for neighbour_opt in neighbours {
            if let Some(neighbour) = neighbour_opt {
                if !closed.contains(&neighbour) {
                    if !candidates.contains(&neighbour) {
                        candidates.push(neighbour);
                    } else {
                        let (i, old_neighbour) = candidates.iter().enumerate().filter(|(_, x)| **x == neighbour).next().unwrap();
                        if neighbour < *old_neighbour {
                            candidates.remove(i);
                            candidates.push(neighbour);
                        }
                    }
                }
            }
        }
        closed.push(current_node);
    }
    if !path_found {
        panic!("PATH not found")
    }
    destiny.history.push(destiny.cord);
    print_path(grid, &destiny.history);
    destiny.cost
}

fn print_path(grid: Vec<Vec<u8>>, path: &Vec<(usize, usize)>) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if path.contains(&(i, j)) {
                green!("{}", grid[i][j]);
            } else {
                print!("{}", grid[i][j]);
            }
        }
        println!();
    }
    println!()
}

fn parse_input(grid_str: String) -> Vec<Vec<u8>> {
    grid_str.lines().into_iter().map(|line| {
        line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>()
    }).collect::<Vec<Vec<u8>>>()
}


#[test]
fn test_case_day15_1() {
    let risk = calculate_lowest_path_risk(
        String::from("1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"));
    assert_eq!(risk, 40)
}

