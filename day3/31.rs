use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn parse_num(line: Vec<char>, i_raw: usize) -> u32 {
    println!("{:?} {}", line, i_raw);

    let mut i = i_raw;

    while i > 0 && line[i-1].is_digit(10) {
        i-= 1;
    }
    println!("i = {}", i);
    let mut res = 0;
    while line[i].is_digit(10) {
        res = res * 10 + line[i].to_digit(10).unwrap();
        println!("res = {}", res);
        i += 1;
        if i == line.len() {
            break;
        }
    }
    println!("parse num {}", res);
    return res;
}

fn check_ns(lines: Vec<Vec<char>>, i: usize, j: usize) -> Vec<u32> {
    let mut res = Vec::new();

    if i != 0 {
        if lines[i-1][j].is_digit(10) {
            res.push(parse_num(lines[i-1].clone(), j));
        } else {
            if j != 0 && lines[i-1][j-1].is_digit(10) {
                res.push(parse_num(lines[i-1].clone(), j-1));
            }
            if j + 1 != lines[i].len() && lines[i-1][j+1].is_digit(10) {
                res.push(parse_num(lines[i-1].clone(), j+1));
            }
        }
    }
    if i + 1 != lines.len() {
        if lines[i+1][j].is_digit(10) {
            res.push(parse_num(lines[i+1].clone(), j));
        } else {
            if j != 0 && lines[i+1][j-1].is_digit(10) {
                res.push(parse_num(lines[i+1].clone(), j-1)); 
            }
            if j + 1 != lines[i].len() && lines[i+1][j+1].is_digit(10) {
                res.push(parse_num(lines[i+1].clone(), j+1));
            }
        }
    }

    if j != 0 && lines[i][j-1].is_digit(10){
        res.push(parse_num(lines[i].clone(), j-1));
    }
    
    if j + 1 != lines[i].len() && lines[i][j+1].is_digit(10){
        res.push(parse_num(lines[i].clone(), j+1));
    }
    return res;
} 

fn main() {
    let lines = lines_from_file("./input");

    let mut res = 0;

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if lines[i][j] != '*' {
                continue;
            }
            let nums = check_ns(lines.clone(), i, j);
            if nums.len() != 2 {
                continue
            }
            println!("{} * {}", nums[0], nums[1]);
            res += nums[0] * nums[1];
        }
    }

    println!("{}", res);
}

fn lines_from_file(
    filename: impl AsRef<Path> + std::convert::AsRef<std::path::Path>,
) -> Vec<Vec<char>> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let vec_string: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    vec_string.iter().map(|l| l.chars().collect()).collect()
}
