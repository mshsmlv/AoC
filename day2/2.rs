use std::fs::File;
use std::io::{self, BufRead};

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

static CUBE_COLORS: &'static [&'static str] = &["red", "green", "blue"];

fn main() {
    let file = File::open("./input").expect("WHERE IS MY DOLORES!");
    let lines = io::BufReader::new(file).lines();

    let mut res = 0;
    let mut possible_games = 0;

    for line in lines {
        let line = line.expect("");
        let mut max_elements_in_bag = [0, 0, 0]; // red, green, blue
        let mut game = line.split(": ");

        let game_num = (&game.nth(0).unwrap()[5..])
            .to_string()
            .parse::<i32>()
            .unwrap();
        for set in game.nth(0).unwrap().split("; ") {
            for cube_num_raw in set.split(", ") {
                let mut num_color = cube_num_raw.split(" ");
                let num = num_color.nth(0).unwrap().parse::<i32>().unwrap();
                let color = num_color.nth(0).unwrap();
                for i in 0..CUBE_COLORS.len() {
                    if color == CUBE_COLORS[i] {
                        if num > max_elements_in_bag[i] {
                            max_elements_in_bag[i] = num;
                        }
                    }
                }
            }
        }
        if max_elements_in_bag[0] <= MAX_RED
            && max_elements_in_bag[1] <= MAX_GREEN
            && max_elements_in_bag[2] <= MAX_BLUE
        {
            possible_games += game_num;
        }
        res += max_elements_in_bag[0] * max_elements_in_bag[1] * max_elements_in_bag[2];
    }

    println!("{}", possible_games);
    println!("{}", res);
}
