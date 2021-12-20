use std::collections::HashMap;
use std::fs::{File};
use std::io::{Read};


fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");
    println!("Total paths: {}", calculate_total_paths(contents));
}

fn calculate_total_paths(contents: String) -> u32 {
    let mut neighbours = parse_neighbours(contents);
    let mut paths_map = HashMap::new();
    let destiny = String::from("end");
    paths_map.insert(destiny.clone(), vec![destiny]);
    let mut visited_nodes = vec![];
    visit_node(&mut neighbours, String::from("start"), &mut paths_map, &mut visited_nodes, false);
    println!("{:?}", paths_map.get("start").unwrap());
    paths_map.get("start").unwrap().len() as u32
}

fn visit_node(neighbours: &mut HashMap<String, Vec<String>>,
              node: String,
              paths_map: &mut HashMap<String, Vec<String>>,
              visited_nodes: &mut Vec<String>,
              visited_twice: bool) {
    if node == String::from("end") {
        return;
    }
    let index: usize = visited_nodes.len();
    if node.chars().nth(0).unwrap().is_lowercase() {
        visited_nodes.insert(index, node.clone());
    }
    let mut paths = Vec::new();
    for neighbour in neighbours.clone().get(&node).unwrap() {
        if !(visited_nodes.contains(neighbour) && visited_twice) && *neighbour != String::from("start") {
            let mut visited_twice = visited_twice;
            if !visited_twice && neighbour.chars().nth(0).unwrap().is_lowercase() && visited_nodes.contains(neighbour) {
                visited_twice = true;
            }
            let mut new_paths = Vec::new();
            visit_node(neighbours, neighbour.clone(), paths_map, visited_nodes, visited_twice);
            for path in paths_map.get(neighbour).unwrap() {
                let new_path = node.clone() + "," + path;
                new_paths.push(new_path);
            }
            paths.append(&mut new_paths);
        }
    }
    paths_map.insert(node.clone(), paths);
    if node.chars().nth(0).unwrap().is_lowercase() {
        visited_nodes.remove(index);
    }
}

fn parse_neighbours(table_str: String) -> HashMap<String, Vec<String>> {
    let mut table = HashMap::new();
    for line in table_str.lines() {
        let mut line_split = line.split("-");
        let key = String::from(line_split.next().unwrap());
        let value = String::from(line_split.next().unwrap().trim());
        if !table.contains_key(&key) {
            table.insert(key.clone(), vec![]);
        }
        table.get_mut(&key).unwrap().push(value.clone());
        if !table.contains_key(&value) {
            table.insert(value.clone(), vec![]);
        }
        table.get_mut(&value).unwrap().push(key.clone());
    }
    table
}

#[test]
fn test_case_day12_2() {
    let points = calculate_total_paths(
        String::from("start-A
start-b
A-c
A-b
b-d
A-end
b-end"));
    assert_eq!(points, 36)
}

#[test]
fn test_case_day12_2_2() {
    let points = calculate_total_paths(
        String::from("start-A
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"));
    assert_eq!(points, 3509)
}