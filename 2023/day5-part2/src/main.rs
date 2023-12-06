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
    let soils = apply_mapping(seed_list, seed_to_soil);
    let ferts = apply_mapping(soils, soil_to_fert);
    let water = apply_mapping(ferts, fert_to_water);
    let light = apply_mapping(water, water_to_light);
    let temp = apply_mapping(light, light_to_temp);
    let hum = apply_mapping(temp, temp_to_hum);
    let locations = apply_mapping(hum, hum_to_loc);
    *locations.iter().min().unwrap()
}

fn apply_mapping(source: Vec<u64>, mappings: Vec<MappingRange>) -> Vec<u64> {
    let mut mapped = std::iter::repeat(false)
        .take(source.len())
        .collect::<Vec<bool>>();
    let mut destination = source.to_vec();
    for mapping in mappings {
        for (i, &item) in source.iter().enumerate() {
            if !mapped[i] && mapping.source <= item && item < mapping.source + mapping.length {
                let shift = item - mapping.source;
                destination[i] = mapping.destination + shift;
                mapped[i] = true;
            }
        }
    }
    destination
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

fn get_seeds_list(line: &str) -> Vec<u64> {
    let x = line.split(':').collect::<Vec<&str>>()[1]
        .split(' ')
        .map(|s| s.trim())
        .filter(|&s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let mut i = 0;
    let mut seeds = vec![];
    while i < x.len() {
        let range = (x[i]..(x[i] + x[i + 1])).collect::<Vec<u64>>();
        seeds.extend(range);
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
        assert_eq!(seeds.len(), 4);
        assert_eq!(seeds[0], 79);
        assert_eq!(seeds[1], 14);
        assert_eq!(seeds[2], 55);
        assert_eq!(seeds[3], 13);
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
