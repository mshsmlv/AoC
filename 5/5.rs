use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

static MAP_NAMES: &'static [&'static str] = &[
    "seed-to-soil",
    "soil-to-fertilizer",
    "fertilizer-to-water",
    "water-to-light",
    "light-to-temperature",
    "temperature-to-humidity",
    "humidity-to-location",
];

#[derive(Debug, Clone)]
struct Range {
    source_start: u64,
    destination_start: u64,
    length: u64,
}

fn parse_num(raw_num: String) -> u64 {
    let mut res = 0;
    for c in raw_num.chars() {
        if !c.is_digit(10) {
            continue;
        }
        res = res * 10 + c.to_digit(10).unwrap();
    }
    return res.into();
}

fn parse_seeds(line: String) -> Vec<u64> {
    let mut seeds_raw = line.split(": ");
    seeds_raw.nth(0);

    let nums_raw = seeds_raw.nth(0).unwrap().split(" ");
    let mut res = vec![0; nums_raw.clone().count()];

    let mut seed_range_start = 0;
    let mut seed_range_length = 0;
    for (i, num_str) in nums_raw.enumerate() {
        res[i] = parse_num(num_str.to_string());
    }
    return res;
}

fn parse_range(line: String) -> Range {
    let mut res = Range {
        source_start: 0,
        destination_start: 0,
        length: 0,
    };

    let mut nums_raw = line.split(" ");
    res.destination_start = parse_num(nums_raw.nth(0).unwrap().to_string());
    res.source_start = parse_num(nums_raw.nth(0).unwrap().to_string());
    res.length = parse_num(nums_raw.nth(0).unwrap().to_string());

    return res;
}

fn get_map_from_range(ranges: Vec<Range>, source: u64) -> u64 {
    for range in ranges.iter() {
        if range.source_start <= source && source < range.source_start + range.length {
            return range.destination_start + (source - range.source_start);
        }
    }
    return source;
}

fn range_to_ranges(ranges: Vec<Range>, source: (u64, u64)) -> Vec<(u64, u64)> {
    let mut res: Vec<(u64, u64)> = Vec::new();

    let start = source.0;
    let len = source.1;

    for range in ranges.iter() {
        // Check entire;
        if range.source_start <= start && start + len <= range.source_start + range.length {
            res.push((
                range.destination_start + (start - range.source_start),
                len,
            ));
            return res;
        }
        // Check range covers begin;
        if range.source_start <= start && start < range.source_start + range.length {
            res.push((
                range.destination_start + (start - range.source_start),
                (range.source_start + range.length - start),
            ));
            res.append(&mut range_to_ranges(
                ranges.clone(),
                (range.source_start + range.length, start + len - (range.source_start + range.length)),
            ));
            return res;
        }
        // Check range covers end;
        if range.source_start < start + len  &&  start + len <= range.source_start + range.length {
            res.push((range.destination_start, (start + len - range.source_start)));
            res.append(&mut range_to_ranges(
                ranges.clone(),
                ((start, range.source_start - start)),
            ));
            return res;
        }

        // Check range inside;
        if  start < range.source_start &&  range.source_start + range.length < start + len {
            res.append(&mut range_to_ranges(
                ranges.clone(),
                ((start, range.source_start - start)),
            ));
            res.push((range.destination_start, range.length));
            res.append(&mut range_to_ranges(
                ranges.clone(),
                ((range.source_start + range.length, start + len - (range.source_start + range.length))),
            ));
        }

    }
    res.push(source);
    return res;
}

fn ranges_to_ranges(ranges: Vec<Range>, sources: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut res = Vec::new();
    for source in sources {
        res.append(&mut range_to_ranges(ranges.clone(), source));
    }
    return res;
}

fn main() {
    let file = File::open("./input").expect("WHERE IS MY DOLORES!");
    let lines = io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("Could not parse line"));

    let mut seeds = Vec::new();
    let mut maps: Vec<Vec<Range>>;

    let mut maps: HashMap<String, Vec<Range>> = HashMap::new();

    let mut current_map: String = "".to_string();

    for (i, line) in lines.enumerate() {
        if i == 0 {
            seeds = parse_seeds(line.clone());
        }
        if line == "" {
            continue;
        }

        if line.as_bytes()[0].is_ascii_digit() {
            if maps.contains_key(&current_map) {
                let map = maps
                    .get_mut(&current_map)
                    .unwrap()
                    .push(parse_range(line.clone()));
            } else {
                let mut new_vec = Vec::new();
                new_vec.push(parse_range(line.clone()));
                maps.insert(current_map.clone(), new_vec);
            }
        }

        for i in 0..MAP_NAMES.len() {
            if line.starts_with(MAP_NAMES[i]) {
                current_map = MAP_NAMES[i].to_string();
            }
        }
    }

    let mut min_location = u64::MAX;

    let mut seed_ranges = Vec::new();
    for i in 0..seeds.len() {
        if i % 2 == 0 {
            seed_ranges.push((seeds[i], seeds[i + 1]));
        }
    }

    for seed_range in seed_ranges {
        let mut current_mapping = Vec::new();
        current_mapping.push(seed_range);
        for map_name in MAP_NAMES {
            let ranges = maps.get(&map_name.to_string()).unwrap().clone();
            current_mapping = ranges_to_ranges(ranges.to_vec(), current_mapping);
        }
        for location in current_mapping.iter() {
            if location.0 < min_location {
                min_location = location.0;
            }
        }
    }

    // for seed in seeds  {
    //     let mut current_mapping = seed;
    //     for map_name in MAP_NAMES {
    //         let ranges = maps.get(&map_name.to_string()).unwrap().clone();
    //         current_mapping = get_map_from_range(ranges.to_vec(), current_mapping);
    //     }
    //     if current_mapping < min_location {
    //         min_location = current_mapping;
    //     }

    // }

    println!("min location {}", min_location);
}
