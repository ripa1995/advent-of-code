use std::{
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

use crate::open_file;

pub fn sum_priorities() -> u64 {
    let file = open_file(
        String::from_str("./inputs/day3.txt").expect("Not a string..."),
    );

    let duplicated = get_duplicated_values(file);

    let sum = duplicated
        .iter()
        .map(|x| calculate_priority_value(*x))
        .sum();

    sum
}

pub fn sum_group_badges() -> u64 {
    let file = open_file(
        String::from_str("./inputs/day3.txt").expect("Not a string..."),
    );

    let badges = get_group_badges(file);

    let sum = badges.iter().map(|x| calculate_priority_value(*x)).sum();

    sum
}

fn get_duplicated_values(file: File) -> Vec<char> {
    let mut duplicated = Vec::new();

    io::BufReader::new(file).lines().for_each(|element| {
        if let Ok(elem) = element {
            let (first, second) = elem.split_at(elem.len() / 2);
            for ch in first.chars() {
                if second.contains(ch) {
                    duplicated.push(ch);
                    break;
                }
            }
        }
    });

    duplicated
}

fn get_group_badges(file: File) -> Vec<char> {
    let mut badges = Vec::new();
    let mut first = String::default();
    let mut second = String::default();

    io::BufReader::new(file).lines().for_each(|element| {
        if let Ok(elem) = element {
            if first.is_empty() {
                first = elem;
            } else if second.is_empty() {
                second = elem;
            } else {
                for ch in first.chars() {
                    if second.contains(ch) && elem.contains(ch) {
                        badges.push(ch);
                        first.clear();
                        second.clear();
                        break;
                    }
                }
            }
        }
    });

    badges
}

fn calculate_priority_value(ch: char) -> u64 {
    if ('A'..='Z').contains(&ch) {
        (ch as u64 % 65) + 27
    } else {
        (ch as u64 % 97) + 1
    }
}
