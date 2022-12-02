use std::{
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

use crate::open_file;

pub fn find_top_n_calories_carried(elf_num: usize) -> u64 {
    let file = open_file(
        String::from_str("./inputs/day1/input.txt").expect("Not a string..."),
    );

    let mut calories = get_grouped_elf_calories(file);

    calories.sort();

    let start = calories.len() - elf_num;
    let end = calories.len();
    let sum = calories[start..end].iter().sum();

    sum
}

fn get_grouped_elf_calories(file: File) -> Vec<u64> {
    let mut calories = Vec::new();
    let mut sum = 0;

    io::BufReader::new(file).lines().for_each(|element| {
        if let Ok(elem) = element {
            if let Ok(elem_num) = elem.parse::<u64>() {
                sum += elem_num;
            } else {
                calories.push(sum);
                sum = 0;
            }
        } else {
            //let's push final elf
            calories.push(sum);
        }
    });

    calories
}
