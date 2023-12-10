use std::fs::File;
use std::io::{self, BufRead};

fn parse_num(raw_num: String) -> u64 {
    let mut res:u64 = 0;
    for c in raw_num.chars() {
        if !c.is_digit(10) {
            continue;
        }
        println!("{}", c);
        res = res * 10 + (c.to_digit(10).unwrap() as u64);
    }
    return res.into();
}

fn main() {
    let file = File::open("./input").expect("WHERE IS MY DOLORES!");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("Could not parse line")).collect();

    let mut times = Vec::new();
    let mut distances = Vec::new();
    // for time_str in lines[0][11..].split_whitespace() {
    //     times.push(parse_num(time_str.to_string()));
    // }
    // for distance_str in lines[1][11..].split_whitespace() {
    //     distances.push(parse_num(distance_str.to_string()));
    // }

    times.push(parse_num(lines[0][11..].replace (" ", "")));

    println!("{:?}", times);
    distances.push(parse_num(lines[1][11..].replace (" ", "")));


    let mut res: u64 = 1;
    for i in 0..times.len() {
        let mut win_ways = 0;
        for speed in 1..times[i] {
            if speed * (times[i] - speed) > distances[i] {
                win_ways +=1;
            }
        }
        res *= win_ways;
    }
    println!("{}", res);

}