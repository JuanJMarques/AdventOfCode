use std::fs::File;
use std::io::Read;


fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let mut vents = Vec::new();
    let mut max: usize = 0;
    for mut line in contents.lines() {
        let mut split = line.split("->");
        let origin = split.next().map(parse_coordinates).expect("Could not parse line");
        let end = split.next().map(parse_coordinates).expect("Could not parse line");
        max = max.max(origin.0 as usize)
                .max(origin.1 as usize)
                .max(end.0 as usize)
                .max(end.1 as usize);
        vents.push((origin, end));
    }
    let mut grid = vec![vec![0; max + 1]; max + 1];
    for (origin, end) in vents {
        if origin.0 == end.0 || origin.1 == end.1 {
            for x in origin.0.min(end.0)..end.0.max(origin.0) + 1 {
                for y in origin.1.min(end.1)..end.1.max(origin.1) + 1 {
                    grid[y as usize][x as usize] += 1;
                }
            }
        }
    }
    println!("{}", grid.iter().map(|x|
            x.iter()
                    .map(|&x| format!("{:>4}", x))
                    .fold(String::new(), |a, b| a + &*b)
    ).fold(String::new(), |a, b| a + "\n" + &*b));
    println!();
    let result = grid.iter().map(|row| row.iter().filter(|&&x| x >= 2).count()).sum::<usize>();
    println!("result: {}", result);
}

fn parse_coordinates(line: &str) -> (i32, i32) {
    let mut split = line.trim().split(",");
    let x = split.next().unwrap().trim().parse::<i32>().unwrap();
    let y = split.next().unwrap().trim().parse::<i32>().unwrap();
    (x, y)
}