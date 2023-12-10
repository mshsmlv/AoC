use std::fs::File;
use std::io::{self, BufRead};
use std::cmp::Ordering;
use std::collections::HashMap;

const HIGH_CARD: usize = 0;
const ONE_PAIR: usize = 1;
const TWO_PAIR: usize = 2;
const THREE_OF_KIND: usize = 3;
const FULL_HOUSE: usize = 4;
const FOUR_OF_KIND: usize = 5;
const FIVE_OF_KIND: usize = 6;

#[derive(PartialEq, Eq, PartialOrd)]
struct Hand {
    cards: String,
    bid: u64,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let rule: String = "AKQT98765432J".to_string();
        for (i, c) in self.cards.chars().enumerate() {
            if c == other.cards.as_bytes()[i] as char {
                continue;
            }
            let self_index = rule.chars().position(|x| x == c).unwrap();
            let other_index = rule.chars().position(|x| x == other.cards.as_bytes()[i] as char).unwrap();

            if self_index > other_index {
                return Ordering::Greater;
            } else {
                return Ordering::Less;
            }
        }
        return Ordering::Equal;
    }
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

fn parse_hand(line: String) -> Hand {
    let mut res = Hand {
        cards: "".to_string(),
        bid: 0,
    };

    let mut raw = line.split(" ");
    res.cards = raw.nth(0).unwrap().to_string();
    res.bid = parse_num(raw.nth(0).unwrap().to_string());
    return res;
}

fn hand_to_type(hand: String) -> usize {
    let mut letters = HashMap::new();

    for c in hand.chars() {
        let stat = letters.entry(c).or_insert(0);
        *stat += 1;
    }
    
    if letters.len() == 5 {
        if letters.contains_key(&'J') {
            return ONE_PAIR;
        }

        return HIGH_CARD;
    }

    if letters.len() == 4 {
        if letters.contains_key(&'J') {
            return THREE_OF_KIND;
        }
        return ONE_PAIR;
    }

    if letters.len() == 3 {
        for (_, num) in &letters {
            if *num == 3 {
                if letters.contains_key(&'J') {
                    return FOUR_OF_KIND;
                } else {
                    return THREE_OF_KIND;
                }
            }
        }

        if letters.contains_key(&'J') {
            let j_num = letters.get(&'J').unwrap();
            if *j_num == 2 {
                return FOUR_OF_KIND;
            } else {
                return FULL_HOUSE;
            }
        }
        
        return TWO_PAIR;
    }

    if letters.len() == 2 {
        for (_, num) in &letters {
            if *num == 4 {
                if letters.contains_key(&'J') {
                    return FIVE_OF_KIND;
                }
                return FOUR_OF_KIND;
            }
        }
        if letters.contains_key(&'J') {
            return FIVE_OF_KIND;
        }
        return FULL_HOUSE;
    }

    if letters.len() == 1 {
        return FIVE_OF_KIND;
    }

    return 0; // unreachable 
}

fn main() {
    let file = File::open("./input").expect("WHERE IS MY DOLORES!");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("Could not parse line")).collect();

    let mut all_hands: Vec<Vec<Hand>> = Vec::new();
    for _ in 0..7 {
        all_hands.push(Vec::new());
    }

    for line in lines {
        let new_hand = parse_hand(line);
        all_hands[hand_to_type(new_hand.cards.clone())].push(new_hand);
    }

    for hands_with_type in &mut all_hands {
        hands_with_type.sort_by(|a, b| b.cmp(a));
    }

    let mut res = 0;
    let mut current_rank = 1;

    for hands_with_type in &mut all_hands {
        for hand in hands_with_type{
            res += hand.bid * current_rank;
            current_rank += 1;
        }
    }

    println!("res {}", res);
}