use std::collections::HashMap;

pub fn connect_junction_boxes(input: String, max_steps: u64) -> u64 {
    let mut circuits = parse_input(input);
    let distances = calculate_distances(circuits.clone());
    let circuits = group_closest_circuits(max_steps, &mut circuits, distances);
    let circuits_sizes = calculate_sizes(circuits);
    let mut circuits_sizes = circuits_sizes.into_values()
        .collect::<Vec<u32>>();
    circuits_sizes.sort();
    circuits_sizes.reverse();
    circuits_sizes.resize(3,1);
    circuits_sizes.iter().fold(1, |acc, &elem| acc * elem as u64)
}

fn calculate_sizes(circuits: HashMap<(u32, u32, u32), u32>) -> HashMap<u32,u32> {
    let mut circuit_sizes = HashMap::new();
    circuits.iter().for_each(|(_coord, circuit)| {
        let size = circuit_sizes.get(circuit).unwrap_or(&0u32) + 1;
        circuit_sizes.insert(*circuit, size);
    });
    circuit_sizes
}

fn group_closest_circuits(max_steps: u64, circuits: &mut HashMap<(u32, u32, u32), u32>, distances: Vec<((u32, u32, u32), (u32, u32, u32), u32)>) -> HashMap<(u32, u32, u32), u32> {
    for i in 0..max_steps {
        let (origin, destination, _distance) = distances[i as usize];
        let &circuit_1 = circuits.get(&origin).unwrap();
        let &circuit_2 = circuits.get(&destination).unwrap();
        if circuit_1 != circuit_2 {
            circuits.insert(destination, circuit_1);
            circuits.iter_mut().for_each(|(_coord, circuit)| {
                if *circuit == circuit_2 {
                    *circuit = circuit_1;
                }
            })
        }
    }
    circuits.clone()
}

fn calculate_distances(circuits: HashMap<(u32, u32, u32), u32>) -> Vec<((u32, u32, u32), (u32, u32, u32), u32)> {
    let circuit_keys = circuits.into_keys().collect::<Vec<(u32, u32, u32)>>();
    let mut distances = vec![];
    for i in 0..circuit_keys.len() - 1 {
        let origin = circuit_keys[i];
        for j in i + 1..circuit_keys.len() {
            let destination = circuit_keys[j];
            let distance = (((destination.0 as i64 - origin.0 as i64).pow(2)
                + (destination.1 as i64 - origin.1 as i64).pow(2)
                + (destination.2 as i64 - origin.2 as i64).pow(2))as f64)
                .sqrt() as u32;
            distances.push((origin,destination,distance));
        }
    }
    distances.sort_by(|a,b| a.2.cmp(&b.2));
    distances
}

fn parse_input(input: String) -> HashMap<(u32, u32, u32), u32> {
    let mut circuits = HashMap::new();
    for (i,line) in input.lines().enumerate() {
        let coords_vec = line.split(',').map(|part| part.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let coords = (coords_vec[0], coords_vec[1], coords_vec[2]);
        circuits.insert(coords, i as u32);
    }
    circuits
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
            .to_string();
        let result = connect_junction_boxes(input,10);
        assert_eq!(result, 40);
    }
}