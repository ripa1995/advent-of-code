use std::{
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

use crate::open_file;

pub fn get_overlapping_assignemnt_number() -> u64 {
    let file = open_file(
        String::from_str("./inputs/day4/input.txt").expect("Not a string..."),
    );

    count_assignment(file, &check_overlapping)
}

fn count_assignment(
    file: File,
    check: &dyn Fn(u64, u64, u64, u64) -> bool,
) -> u64 {
    let mut num = 0;

    io::BufReader::new(file).lines().for_each(|element| {
        if let Ok(elem) = element {
            if let Some((first, second)) = elem.split_once(',') {
                if let Some((low_first, high_first)) = first.split_once('-') {
                    if let Some((low_second, high_second)) =
                        second.split_once('-')
                    {
                        if let Ok(low_first) = low_first.parse::<u64>() {
                            if let Ok(high_first) = high_first.parse::<u64>() {
                                if let Ok(low_second) =
                                    low_second.parse::<u64>()
                                {
                                    if let Ok(high_second) =
                                        high_second.parse::<u64>()
                                    {
                                        if check(
                                            low_first,
                                            low_second,
                                            high_first,
                                            high_second,
                                        ) {
                                            num += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    num
}

fn check_fully_contained(
    low_first: u64,
    low_second: u64,
    high_first: u64,
    high_second: u64,
) -> bool {
    (low_first >= low_second && high_first <= high_second)
        || (low_first <= low_second && high_first >= high_second)
}

fn check_overlapping(
    low_first: u64,
    low_second: u64,
    high_first: u64,
    high_second: u64,
) -> bool {
    ((low_first >= low_second && low_first <= high_second) || (low_first <= low_second && high_first >= low_second))
    || check_fully_contained(low_first, low_second, high_first, high_second)
}
