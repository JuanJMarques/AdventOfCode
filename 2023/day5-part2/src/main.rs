use std::fs::File;
use std::io::Read;

enum MappingType {
    Seed,
    SeedSoil,
    SoilFert,
    FertWater,
    WaterLight,
    LightTemp,
    TempHum,
    HumLoc,
}

struct MappingRange {
    source: u64,
    destination: u64,
    length: u64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Range {
    init: u64,
    end: u64,
}

fn main() {
    let lower_location = get_lower_location_number(read_input_file());
    println!("the lower location number is {}", lower_location);
}

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
}

fn get_lower_location_number(input: String) -> u64 {
    let mut mapping_type = MappingType::Seed;
    let mut seed_list = vec![];
    let mut seed_to_soil = vec![];
    let mut soil_to_fert = vec![];
    let mut fert_to_water = vec![];
    let mut water_to_light = vec![];
    let mut light_to_temp = vec![];
    let mut temp_to_hum = vec![];
    let mut hum_to_loc = vec![];
    for line in input.lines() {
        if line.is_empty() {
            mapping_type = get_next_mapping_type(mapping_type);
        } else {
            match mapping_type {
                MappingType::Seed => {
                    seed_list = get_seeds_list(line);
                }
                MappingType::SeedSoil => {
                    if !line.contains("map") {
                        seed_to_soil.push(parse_mapping_range(line))
                    }
                }
                MappingType::SoilFert => {
                    if !line.contains("map") {
                        soil_to_fert.push(parse_mapping_range(line))
                    }
                }
                MappingType::FertWater => {
                    if !line.contains("map") {
                        fert_to_water.push(parse_mapping_range(line))
                    }
                }
                MappingType::WaterLight => {
                    if !line.contains("map") {
                        water_to_light.push(parse_mapping_range(line))
                    }
                }
                MappingType::LightTemp => {
                    if !line.contains("map") {
                        light_to_temp.push(parse_mapping_range(line))
                    }
                }
                MappingType::TempHum => {
                    if !line.contains("map") {
                        temp_to_hum.push(parse_mapping_range(line))
                    }
                }
                MappingType::HumLoc => {
                    if !line.contains("map") {
                        hum_to_loc.push(parse_mapping_range(line))
                    }
                }
            }
        }
    }
    let soils = apply_mappings(seed_list, seed_to_soil);
    let ferts = apply_mappings(soils, soil_to_fert);
    let water = apply_mappings(ferts, fert_to_water);
    let light = apply_mappings(water, water_to_light);
    let temp = apply_mappings(light, light_to_temp);
    let hum = apply_mappings(temp, temp_to_hum);
    let locations = apply_mappings(hum, hum_to_loc);
    *locations
        .iter()
        .map(|Range { init, end: _ }| init)
        .min()
        .unwrap()
}

fn apply_mappings(source: Vec<Range>, mappings: Vec<MappingRange>) -> Vec<Range> {
    let mut mapped = std::iter::repeat(false)
        .take(source.len())
        .collect::<Vec<bool>>();
    let mut destination = source.to_vec();
    for mapping in mappings {
        (destination, mapped) = apply_mapping(destination.clone(), mapping, mapped.clone());
    }
    destination.clone()
}

fn apply_mapping(
    source: Vec<Range>,
    mapping: MappingRange,
    mapped: Vec<bool>,
) -> (Vec<Range>, Vec<bool>) {
    let mut destination = vec![];
    let mut new_mapped = vec![];
    let MappingRange {
        source: init,
        destination: dest,
        length,
    } = mapping;
    let end = init + length - 1;
    for (i, &Range { init: a, end: b }) in source.iter().enumerate() {
        if !mapped[i] {
            let inner_dif = b - a;
            if b < init {
                destination.push(Range { init: a, end: b });
                new_mapped.push(false);
            }
            if init <= a {
                let diff = a - init;
                if end < a {
                    destination.push(Range { init: a, end: b });
                    new_mapped.push(false);
                }
                if b <= end {
                    destination.push(Range {
                        init: dest + diff,
                        end: dest + diff + inner_dif,
                    });
                    new_mapped.push(true);
                }
                if a <= end && end < b {
                    let inner_dif = end - a;
                    destination.push(Range {
                        init: dest + diff,
                        end: dest + diff + inner_dif,
                    });
                    new_mapped.push(true);
                    destination.push(Range {
                        init: end + 1,
                        end: b,
                    });
                    new_mapped.push(false);
                }
            }
            if a < init && init <= b {
                if b <= end {
                    destination.push(Range {
                        init: a,
                        end: init - 1,
                    });
                    new_mapped.push(false);
                    let inner_dif = b - init;
                    destination.push(Range {
                        init: dest,
                        end: dest + inner_dif,
                    });
                    new_mapped.push(true);
                } else {
                    destination.push(Range {
                        init: a,
                        end: init - 1,
                    });
                    new_mapped.push(false);
                    let inner_dif = b - end;
                    destination.push(Range {
                        init: dest,
                        end: dest + inner_dif,
                    });
                    new_mapped.push(true);
                    destination.push(Range {
                        init: end + 1,
                        end: b,
                    });
                    new_mapped.push(false);
                }
            }
        } else {
            destination.push(Range { init: a, end: b });
            new_mapped.push(true);
        }
    }
    (destination, new_mapped)
}

fn parse_mapping_range(line: &str) -> MappingRange {
    let data = line
        .split(' ')
        .map(|s| s.trim())
        .filter(|&s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    MappingRange {
        source: data[1],
        destination: data[0],
        length: data[2],
    }
}

fn get_seeds_list(line: &str) -> Vec<Range> {
    let x = line.split(':').collect::<Vec<&str>>()[1]
        .split(' ')
        .map(|s| s.trim())
        .filter(|&s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let mut i = 0;
    let mut seeds = vec![];
    while i < x.len() {
        let range = Range {
            init: x[i],
            end: x[i] + x[i + 1] - 1,
        };
        seeds.push(range);
        i += 2
    }
    seeds
}

fn get_next_mapping_type(actual: MappingType) -> MappingType {
    match actual {
        MappingType::Seed => MappingType::SeedSoil,
        MappingType::SeedSoil => MappingType::SoilFert,
        MappingType::SoilFert => MappingType::FertWater,
        MappingType::FertWater => MappingType::WaterLight,
        MappingType::WaterLight => MappingType::LightTemp,
        MappingType::LightTemp => MappingType::TempHum,
        MappingType::TempHum => MappingType::HumLoc,
        MappingType::HumLoc => MappingType::Seed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_seeds() {
        let seeds = get_seeds_list("seeds: 79 14 55 13");
        assert_eq!(seeds.len(), 2);
        assert_eq!(seeds[0].init, 79);
        assert_eq!(seeds[0].end, 79 + 13);
        assert_eq!(seeds[1].init, 55);
        assert_eq!(seeds[1].end, 55 + 12);
    }

    #[test]
    fn test_apply_mapping() {
        let original = vec![Range { init: 10, end: 20 }];
        let mapping = MappingRange {
            source: 0,
            destination: 21,
            length: 10,
        };
        let (mapped, applied) = apply_mapping(original.clone(), mapping, vec![false]);
        assert_eq!(1, mapped.len());
        assert_eq!(1, applied.len());
        assert!(!applied[0]);
        assert_eq!(original.clone(), mapped);
        let mapping = MappingRange {
            source: 0,
            destination: 21,
            length: 11,
        };
        let (mapped, applied) = apply_mapping(original.clone(), mapping, vec![false]);
        assert_eq!(2, mapped.len());
        assert_eq!(2, applied.len());
        assert!(applied[0]);
        assert!(!applied[1]);
        assert_eq!(mapped[0], Range { init: 31, end: 31 });
        assert_eq!(mapped[1], Range { init: 11, end: 20 });
        let mapping = MappingRange {
            source: 0,
            destination: 21,
            length: 15,
        };
        let (mapped, applied) = apply_mapping(original.clone(), mapping, vec![false]);
        assert_eq!(2, mapped.len());
        assert_eq!(2, applied.len());
        assert!(applied[0]);
        assert!(!applied[1]);
        assert_eq!(mapped[0], Range { init: 31, end: 35 });
        assert_eq!(mapped[1], Range { init: 15, end: 20 });
        let mapping = MappingRange {
            source: 0,
            destination: 21,
            length: 1000,
        };
        let (mapped, applied) = apply_mapping(original.clone(), mapping, vec![false]);
        assert_eq!(1, mapped.len());
        assert!(applied[0]);
        assert_eq!(mapped[0], Range { init: 31, end: 41 });
        let mapping = MappingRange {
            source: 15,
            destination: 21,
            length: 1000,
        };
        let (mapped, applied) = apply_mapping(original.clone(), mapping, vec![false]);
        assert_eq!(2, mapped.len());
        assert_eq!(2, applied.len());
        assert!(!applied[0]);
        assert!(applied[1]);
        assert_eq!(mapped[0], Range { init: 10, end: 14 });
        assert_eq!(mapped[1], Range { init: 15, end: 26 });
    }

    #[test]
    fn test_input() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            .to_string();
        let result = get_lower_location_number(input);
        assert_eq!(result, 46);
    }
}
