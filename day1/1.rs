use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("./input").expect("WHERE IS MY DOLORES!");
    let lines = io::BufReader::new(file).lines();

    let spell_digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut res = 0;
    for line in lines {
        let line = line.expect("");
        let mut first_num = 0;
        let mut first_num_is_found = false;
        let mut last_num = 0;

        for i in 0..line.len() { 
            if line.chars().nth(i).unwrap().is_digit(10) {
                if !first_num_is_found {
                    first_num = line.chars().nth(i).unwrap().to_digit(10).unwrap();
                    first_num_is_found = true;
                }
                last_num = line.chars().nth(i).unwrap().to_digit(10).unwrap();
            } else {
                for j in 0..spell_digits.len() {                  
                    if (i + spell_digits[j].len()) > line.len() {
                        continue;
                    }

                    if spell_digits[j] == (&line[i..i+spell_digits[j].len()]).to_string() {
                        if !first_num_is_found {
                            first_num = (j as u32) +1;
                            first_num_is_found = true;
                        }
                        last_num = (j as u32)+1;
                    }
                }
            }


        }
        res += first_num * 10 + last_num;
    }
    println!("{}", res);
}
