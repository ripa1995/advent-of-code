use std::{
    fs::File,
    io::{self, BufRead},
    str::FromStr, string::ParseError,
};

mod tournament_match;

use crate::open_file;


pub fn find_max_possible_score() -> u64 {
    let file = open_file(
        String::from_str("./inputs/day2/input.txt").expect("Not a string..."),
    );

    let max_score = get_max_score(file);

    max_score
}

fn get_max_score(file: File) -> u64 {
    let mut sum = 0;

    io::BufReader::new(file).lines().for_each(|element| {
        if let Ok(elem) = element {
           if let Ok(res) = tournament_match::RPSMatch::from_str(&elem) {
                sum += res.eval_match() as u64;
           }
        }
    });

    sum
}

pub fn find_max_possible_score_v2() -> u64 {
    let file = open_file(
        String::from_str("./inputs/day2/input.txt").expect("Not a string..."),
    );

    let max_score = get_max_score_v2(file);

    max_score
}

fn get_max_score_v2(file: File) -> u64 {
    let mut sum = 0;

    io::BufReader::new(file).lines().for_each(|element| {
        if let Ok(elem) = element {
           if let Ok(res) = tournament_match::RPSMatchV2::from_str(&elem) {
                sum += res.eval_match() as u64;
           }
        }
    });

    sum
}