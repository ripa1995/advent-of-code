use std::{
    collections::{VecDeque, HashSet},
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

use crate::open_file;

pub fn find_index_in_msg() -> usize {
    let file = open_file(
        String::from_str("./inputs/day6/input.txt").expect("Not a string..."),
    );

    find_first_start_of_message(file)
}

fn find_first_start_of_packet(file: File) -> usize {
    let mut index = 0;

    io::BufReader::new(file).lines().for_each(|element| {
        if let Ok(elem) = element {
            let chars = elem.as_bytes();
            for i in 0..chars.len() - 3 {
                if chars[i] != chars[i + 1]
                    && chars[i] != chars[i + 2]
                    && chars[i] != chars[i + 3]
                    && chars[i + 1] != chars[i + 2]
                    && chars[i + 1] != chars[i + 3]
                    && chars[i + 2] != chars[i + 3]
                    && index == 0
                {
                    index = i + 4;
                }
            }
        }
    });

    index
}

fn find_first_start_of_message(file: File) -> usize {
    let mut index = 0;
    let mut set = HashSet::new();

    io::BufReader::new(file).lines().for_each(|element| {
        if let Ok(elem) = element {
            let chars = elem.as_bytes();
            for i in 0..chars.len() - 13 {
                for j in i..i+14 {
                   if !set.insert(chars[j]) {
                    set.clear();
                    break;
                   }
                }
                if set.len() == 14 {
                    index = i + 14;
                    break;
                }
            }
        }
    });

    index
}
