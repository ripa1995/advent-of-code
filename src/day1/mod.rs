use std::{fs::File, str::FromStr, io::{self, BufRead}};

pub fn find_most_calories_carried() -> u64 {
    let file = open_file(String::from_str("./inputs/day1/input.txt").expect("Not a string..."));
        
    let mut max = 0;
    let mut sum = 0;
    
    io::BufReader::new(file).lines().for_each(|element| {
        if let Ok(elem) = element {
            if let Ok(elem_num) = elem.parse::<u64>() {
                sum += elem_num;
            } else {
                max = check_max(max, sum);
                sum = 0;
            }
        }
    });
    max = check_max(max, sum);
    max
}

fn check_max(max: u64, sum: u64) -> u64 {
    if max < sum {
        sum
    } else {
        max
    }
}

fn open_file(path: String) -> File {
    File::open(path)
    .expect("Failed to read the file...")
}