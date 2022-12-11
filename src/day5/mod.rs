use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

use crate::open_file;

struct Supplies {
    pub stacks: Vec<Vec<char>>,
}

struct Instructions {
    pub qty: usize,
    pub from: usize,
    pub to: usize,
}

impl FromStr for Instructions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = Self {
            qty: 0,
            from: 0,
            to: 0,
        };
        if s.contains("move") {
            s.split_whitespace().enumerate().for_each(|(i, val)| {
                if i == 1 {
                    res.qty = val.parse().unwrap();
                } else if i == 3 {
                    res.from = val.parse().unwrap();
                } else if i == 5 {
                    res.to = val.parse().unwrap();
                }
            });
            Ok(res)
        } else {
            Err(String::new())
        }
    }
}

pub fn rearrange_supplies(multiple_crates: bool) {
    let file = open_file(
        String::from_str("./inputs/day5/input.txt").expect("Not a string..."),
    );

    let (mut stacks, instructions) = get_info(file);

    instructions.iter().for_each(|elem| {
        let mut temp_vec = VecDeque::new();
        for i in 0..elem.qty {
            if let Some(temp) = stacks[elem.from-1].pop_back() {
                if multiple_crates {
                    temp_vec.push_front(temp);
                } else {
                    stacks[elem.to-1].push_back(temp);
                }
            }
        }
        if multiple_crates {
            stacks[elem.to-1].append(&mut temp_vec);
        }
    });

    stacks
        .iter()
        .for_each(|x| print!("{}", x.back().unwrap_or(&' ')))
}

fn get_info(file: File) -> ([VecDeque<char>; 9], Vec<Instructions>) {
    let mut stacks = [
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
    ];
    let mut instructions = Vec::new();

    io::BufReader::new(file).lines().for_each(|element| {
        if let Ok(elem) = element {
            if elem.contains('[') {
                elem.chars().enumerate().for_each(|(i, val)| {
                    if val.is_alphabetic() {
                        stacks[i / 4].push_front(val);
                    }
                });
            } else if elem.contains(" 1   2") {
                //do nothing
            } else {
                if !elem.is_empty() {
                    instructions.push(Instructions::from_str(&elem).unwrap())
                }
            }
        }
    });

    (stacks, instructions)
}
