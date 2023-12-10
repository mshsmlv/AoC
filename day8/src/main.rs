use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use num::Integer;

fn parse_node(line: String) -> (String, (String, String)) {
    let mut raw = line.split(" = ");
    let key = raw.nth(0).unwrap().to_string();
    
    let binding = raw.nth(0).unwrap().to_string();
    let mut pair_raw =  binding.split(", ");
    let left = pair_raw.nth(0).unwrap().to_string().strip_prefix("(").unwrap().to_string();
    let right =  pair_raw.nth(0).unwrap().to_string().strip_suffix(")").unwrap().to_string();

    return (key, (left, right));
}

fn main() {
    let file = File::open("./input").expect("WHERE IS MY DOLORES!");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("Could not parse line")).collect();

    let mut graph = HashMap::new();
    
    let steps = &lines[0];

    let mut start_nodes = Vec::new();

    for i in 2..lines.len() {
        let (key, pair) = parse_node(lines[i].clone());
        if key.ends_with("A") {
            start_nodes.push(key.clone());
        }
        graph.insert(key, pair);
    }



    let mut steps_for_all_points = Vec::new();

    for node in start_nodes {
        let mut res: u64 = 0;
        let mut current_node = node;
        while !current_node.ends_with("Z") {
            for (_, c) in steps.chars().enumerate() {
                res += 1;
                let pair = graph.get(&current_node).unwrap();
                if c == 'L' {
                    current_node = pair.0.clone();
                }
                if c == 'R' {
                    current_node = pair.1.clone();
                }
            }
        }
        steps_for_all_points.push(res);
    }
    
    println!("res {}", steps_for_all_points.iter().fold(1u64, |acc, x| acc.lcm(&x)));
}