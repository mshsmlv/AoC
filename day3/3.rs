use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let lines = lines_from_file("./input");

    let mut prev_is_symbol = false;
    let mut num_is_needed = false;
    let mut current_num = 0;
    let mut res = 0;

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if lines[i][j] == '.' {
                if current_num != 0 {
                    if num_is_needed == false {
                        if i != 0 {
                            if !lines[i - 1][j].is_digit(10) && lines[i - 1][j] != '.' {
                                num_is_needed = true;
                            }
                        }
                        if i + 1 != lines.len() {
                            if !lines[i + 1][j].is_digit(10) && lines[i + 1][j] != '.' {
                                num_is_needed = true;
                            }
                        }
                    }

                    if num_is_needed {
                        res += current_num;
                    }

                    current_num = 0;
                    num_is_needed = false;
                }
                prev_is_symbol = false;
                continue;
            }

            if !lines[i][j].is_digit(10) {
                res += current_num;
                current_num = 0;
                num_is_needed = false;
                prev_is_symbol = true;
                continue;
            }

            if prev_is_symbol {
                num_is_needed = true;
                prev_is_symbol = false;
            }

            if i != 0 && j != 0 {
                if !lines[i - 1][j - 1].is_digit(10) && lines[i - 1][j - 1] != '.' {
                    num_is_needed = true;
                }
            }
            if i + 1 != lines.len() && j != 0 {
                if !lines[i + 1][j - 1].is_digit(10) && lines[i + 1][j - 1] != '.' {
                    num_is_needed = true;
                }
            }

            if i != 0 && !lines[i - 1][j].is_digit(10) && lines[i - 1][j] != '.' {
                num_is_needed = true;
            }
            if i + 1 != lines.len() && !lines[i + 1][j].is_digit(10) && lines[i + 1][j] != '.' {
                num_is_needed = true;
            }

            current_num = current_num * 10 + lines[i][j].to_digit(10).unwrap();
            println!("{} {}", current_num, num_is_needed);
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
