use std::collections::HashMap;
use std::io;
use rand::Rng;

fn main() {
    println!("Type anything for morse conversion");
    println!("Type !exit to exit");
    let morse_table = HashMap::from([
        ('a', Vec::from([false, true])),
        ('b', Vec::from([true, false, false, false])),
        ('c', Vec::from([true, false, true, false])),
        ('d', Vec::from([true, false, false])),
        ('e', Vec::from([false])),
        ('f', Vec::from([false, false, true, false])),
        ('g', Vec::from([true, true, false])),
        ('h', Vec::from([false, false, false, false])),
        ('i', Vec::from([false, false])),
        ('j', Vec::from([false, true, true, true])),
        ('k', Vec::from([true, false, true])),
        ('l', Vec::from([false, true, false, false])),
        ('m', Vec::from([true, true])),
        ('n', Vec::from([true, false])),
        ('o', Vec::from([true, true, true])),
        ('p', Vec::from([false, true, true, false])),
        ('q', Vec::from([true, true, false, true])),
        ('r', Vec::from([false, true, false])),
        ('s', Vec::from([false, false, false])),
        ('t', Vec::from([true])),
        ('u', Vec::from([false, false, true])),
        ('v', Vec::from([false, false, false, true])),
        ('w', Vec::from([false, true, true])),
        ('x', Vec::from([true, false, false, true])),
        ('y', Vec::from([true, false, true, true])),
        ('z', Vec::from([true, true, false, false])),
        ('1', Vec::from([false, true, true, true, true])),
        ('2', Vec::from([false, false, true, true, true])),
        ('3', Vec::from([false, false, false, true, true])),
        ('4', Vec::from([false, false, false, false, true])),
        ('5', Vec::from([false, false, false, false, false])),
        ('6', Vec::from([true, false, false, false, false])),
        ('7', Vec::from([true, true, false, false, false])),
        ('8', Vec::from([true, true, true, false, false])),
        ('9', Vec::from([true, true, true, true, false])),
        ('0', Vec::from([true, true, true, true, true])),
        (' ', Vec::from([false, false, false, false, false, false])),
        ('.', Vec::from([false, false, false, false, false, true])),
        (',', Vec::from([false, false, false, false, true, true])),
        ('?', Vec::from([false, false, true, true, true, true])),
        ('\'', Vec::from([false, true, true, true, true, true])),
        ('!', Vec::from([false, true, false, true, true, true])),
        ('/', Vec::from([true, false, false, true, true, true])),
        ('(', Vec::from([true, true, false, true, true, true])),
        (')', Vec::from([true, true, true, true, true, true])),
        ('&', Vec::from([false, false, true, false, true, true])),
        (':', Vec::from([false, false, false, true, true, true])),
        (';', Vec::from([false, false, false, false, true, true])),
        ('=', Vec::from([true, false, false, false, true, true])),
        ('+', Vec::from([true, false, true, false, true, true])),
        ('-', Vec::from([true, false, true, true, true, true])),
        ('_', Vec::from([false, true, false, true, true, true])),
        ('"', Vec::from([false, true, true, false, true, true])),
        ('$', Vec::from([true, false, false, true, false, true])),
        ('@', Vec::from([true, false, false, true, true, true]))
    ]);
    let mut exit = false;
    let mut buffer = audio::Dynamic::<f32>::new();
    buffer.resize_channels(2);
    buffer.resize(2048);
    let mut rng = rand::thread_rng();
    rng.fill(&mut buffer[0]);
    rng.fill(&mut buffer[1]);
    while !exit {
        let mut read_input = String::new();
        match io::stdin().read_line(&mut read_input) {
            Ok(_) => {
                let line = read_input.trim();
                if "!exit" == line {
                    exit = true;
                    continue;
                }
                for c in line.chars() {
                    if morse_table.contains_key(&c) {
                        let mut output = String::new();
                        morse_table.get(&c).unwrap().iter().for_each(|x| {
                            output.push_str(if *x { "." } else { "-" });
                        });
                        print!("{}", output);
                    } else {
                        println!("Char not found in morse table: {}", c);
                        continue;
                    }
                }
            }
            Err(_) => exit = true,
        }
        println!();
        println!("Type anything for morse conversion");
    }
}