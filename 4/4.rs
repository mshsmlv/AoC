use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

struct Card {
    num: u32,
    win_nums: HashMap<u32, bool>,
    matched_nums: i32,
}

fn parse_num(raw_num: String) -> u32 {
    let mut res = 0;
    for c in raw_num.chars() {
        if !c.is_digit(10) {
            continue;
        }
        res = res * 10 + c.to_digit(10).unwrap();
    }
    return res;
}

fn parse_card(line: String) -> Card {
    let mut res = Card {
        num: 0,
        win_nums: HashMap::new(),
        matched_nums: 0,
    };

    let mut card = line.split(": ");

    res.num =  parse_num(card.nth(0).unwrap().to_string());
    
    let card_content = &mut card.nth(0).unwrap().split(" | ");

    for win_num_str in card_content.nth(0).unwrap().split(" ") {
        if win_num_str.is_empty() {
            continue;
        }
        res.win_nums.insert(parse_num(win_num_str.trim().to_string()), true);
    }

    for real_num_str in card_content.nth(0).unwrap().split(" ") {
        if real_num_str.is_empty() {
            continue;
        }
        if res.win_nums.contains_key(&parse_num(real_num_str.trim().to_string())) {
            res.matched_nums += 1;
        }
    }
    return res;
}

fn main() {
    let file = File::open("./input").expect("WHERE IS MY DOLORES!");
    let lines = io::BufReader::new(file).lines().map(|l| l.expect("Could not parse line"));

    let mut res = 0;

    let mut copies: HashMap<usize, i32> = HashMap::new();

    for (i, line) in lines.enumerate() {
        let card = parse_card(line);
        if !copies.contains_key(&(i+1)) {
            copies.insert(i+1, 1);
        }

        if card.matched_nums != 0 {
            let mut multiple = 1;

            if copies.contains_key(&(i + 1)) {
                multiple = copies.get(&(i+1)).unwrap().clone();
            }
            for k in 0..multiple {
                for j in 0..(card.matched_nums as usize) {
                    if copies.contains_key(&(i+1+j + 1)) {
                        *copies.get_mut(&(i+1+j + 1)).unwrap() += 1;
                    } else {
                        copies.insert(i+1+j + 1, 2);
                    }
                }
            }

            res += 1 << (card.matched_nums - 1);
        }
    }

    let mut res1 = 0;
    for (_, value) in &copies {
        res1 += value;
    }
    println!("res {}", res);
    println!("res1 {}", res1);

}