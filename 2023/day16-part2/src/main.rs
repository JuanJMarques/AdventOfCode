use std::fs::File;
use std::io::Read;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::thread::JoinHandle;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
struct Beam {
    x: usize,
    y: usize,
    direction: Direction,
}

fn main() {
    let count = count_energized_tiles(read_input_file());
    println!("the number of energized tiles is {}", count);
}

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}

fn count_energized_tiles(input: String) -> u32 {
    let mut matrix = vec![vec!['.'; input.lines().next().unwrap().len()]; input.lines().count()];
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            matrix[i][j] = ch;
        }
    }
    let mut threads = Vec::new();
    let (sender, receiver) = channel();
    for y in 0..matrix.len() {
        let init_beam = Beam {
            x: 0,
            y,
            direction: Direction::Right,
        };
        threads.push(energize(
            init_beam,
            vec![vec![vec![]; input.lines().next().unwrap().len()]; input.lines().count()],
            matrix.clone(),
            sender.clone(),
        ));
        let init_beam = Beam {
            x: matrix[0].len() - 1,
            y,
            direction: Direction::Left,
        };
        threads.push(energize(
            init_beam,
            vec![vec![vec![]; input.lines().next().unwrap().len()]; input.lines().count()],
            matrix.clone(),
            sender.clone(),
        ));
    }
    for x in 0..matrix[0].len() {
        let init_beam = Beam {
            x,
            y: 0,
            direction: Direction::Down,
        };
        threads.push(energize(
            init_beam,
            vec![vec![vec![]; input.lines().next().unwrap().len()]; input.lines().count()],
            matrix.clone(),
            sender.clone(),
        ));
        let init_beam = Beam {
            x: 0,
            y: matrix.len() - 1,
            direction: Direction::Up,
        };
        threads.push(energize(
            init_beam,
            vec![vec![vec![]; input.lines().next().unwrap().len()]; input.lines().count()],
            matrix.clone(),
            sender.clone(),
        ));
    }
    let num_threads = threads.len();
    for thread in threads {
        thread.join().expect("");
    }
    let mut max = 0;
    for _ in 0..num_threads {
        if let Ok(x) = receiver.recv() {
            if max < x {
                max = x;
            }
        }
    }
    max
}

fn energize(
    init_beam: Beam,
    energized_tiles: Vec<Vec<Vec<Direction>>>,
    matrix: Vec<Vec<char>>,
    sender: Sender<u32>,
) -> JoinHandle<()> {
    let mut energized_tiles = energized_tiles.clone();
    thread::spawn(move || {
        let mut beams = process_beams(init_beam, &matrix);
        while let Some(beam) = beams.pop() {
            energized_tiles[beam.y][beam.x].push(beam.direction);
            let mut new_beams = step(beam, &matrix);
            new_beams = new_beams
                .iter()
                .filter(|b| !energized_tiles[b.y][b.x].iter().any(|&d| d == b.direction))
                .copied()
                .collect::<Vec<Beam>>();
            beams.append(&mut new_beams);
        }
        sender
            .send(
                energized_tiles
                    .iter()
                    .map(|row| row.iter().filter(|b| !b.is_empty()).count() as u32)
                    .sum::<u32>(),
            )
            .unwrap();
    })
}

fn step(beam: Beam, x: &Vec<Vec<char>>) -> Vec<Beam> {
    let mut beams = Vec::new();
    let x_dir = get_xdir(beam.direction);
    let y_dir = get_ydir(beam.direction);
    if (beam.x as i8 + x_dir) >= 0
        && ((beam.x as i8 + x_dir) as usize) < x[0].len()
        && (beam.y as i8 + y_dir) >= 0
        && ((beam.y as i8 + y_dir) as usize) < x.len()
    {
        let new_x = (beam.x as i8 + x_dir) as usize;
        let new_y = (beam.y as i8 + y_dir) as usize;
        let into = Beam {
            direction: beam.direction,
            x: new_x,
            y: new_y,
        };
        let mut new_beams = process_beams(into, x);
        beams.append(&mut new_beams);
    }
    beams
}

fn process_beams(beam: Beam, matrix: &[Vec<char>]) -> Vec<Beam> {
    match matrix[beam.y][beam.x] {
        '.' => vec![beam],
        '|' => match beam.direction {
            Direction::Up => vec![beam],
            Direction::Down => vec![beam],
            Direction::Left => vec![
                Beam {
                    x: beam.x,
                    y: beam.y,
                    direction: Direction::Up,
                },
                Beam {
                    x: beam.x,
                    y: beam.y,
                    direction: Direction::Down,
                },
            ],
            Direction::Right => vec![
                Beam {
                    x: beam.x,
                    y: beam.y,
                    direction: Direction::Up,
                },
                Beam {
                    x: beam.x,
                    y: beam.y,
                    direction: Direction::Down,
                },
            ],
        },
        '-' => match beam.direction {
            Direction::Left => vec![beam],
            Direction::Right => vec![beam],
            Direction::Up => vec![
                Beam {
                    x: beam.x,
                    y: beam.y,
                    direction: Direction::Left,
                },
                Beam {
                    x: beam.x,
                    y: beam.y,
                    direction: Direction::Right,
                },
            ],
            Direction::Down => vec![
                Beam {
                    x: beam.x,
                    y: beam.y,
                    direction: Direction::Left,
                },
                Beam {
                    x: beam.x,
                    y: beam.y,
                    direction: Direction::Right,
                },
            ],
        },
        '/' => match beam.direction {
            Direction::Left => vec![Beam {
                x: beam.x,
                y: beam.y,
                direction: Direction::Down,
            }],
            Direction::Right => vec![Beam {
                x: beam.x,
                y: beam.y,
                direction: Direction::Up,
            }],
            Direction::Up => vec![Beam {
                x: beam.x,
                y: beam.y,
                direction: Direction::Right,
            }],
            Direction::Down => vec![Beam {
                x: beam.x,
                y: beam.y,
                direction: Direction::Left,
            }],
        },
        '\\' => match beam.direction {
            Direction::Left => vec![Beam {
                x: beam.x,
                y: beam.y,
                direction: Direction::Up,
            }],
            Direction::Right => vec![Beam {
                x: beam.x,
                y: beam.y,
                direction: Direction::Down,
            }],
            Direction::Up => vec![Beam {
                x: beam.x,
                y: beam.y,
                direction: Direction::Left,
            }],
            Direction::Down => vec![Beam {
                x: beam.x,
                y: beam.y,
                direction: Direction::Right,
            }],
        },
        _ => vec![],
    }
}

fn get_xdir(dir: Direction) -> i8 {
    match dir {
        Direction::Left => -1,
        Direction::Right => 1,
        _ => 0,
    }
}

fn get_ydir(dir: Direction) -> i8 {
    match dir {
        Direction::Up => -1,
        Direction::Down => 1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|...."
            .to_string();
        let result = count_energized_tiles(input);
        assert_eq!(result, 51);
    }
}
