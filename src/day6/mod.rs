use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

use crate::open_file;

pub fn find_index_in_msg() -> usize {
    let file = open_file(
        String::from_str("./inputs/day6/input.txt").expect("Not a string..."),
    );

    find(file, 14)
}

fn find(file: File, packet_size: usize) -> usize {
    let mut index = 0;
    let mut set = HashSet::new();

    io::BufReader::new(file).lines().for_each(|element| {
        if let Ok(elem) = element {
            let chars = elem.as_bytes();
            for i in 0..chars.len() - (packet_size - 1) {
                for item in chars.iter().skip(i).take(packet_size) {
                    if !set.insert(*item) {
                        set.clear();
                        break;
                    }
                }
                if set.len() == packet_size {
                    index = i + packet_size;
                    break;
                }
            }
        }
    });

    index
}
