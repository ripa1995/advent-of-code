use std::{
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

use crate::open_file;

pub fn get_visible_trees() {
    let file = open_file(
        String::from_str("./inputs/day8.txt").expect("Not a string..."),
    );

    let mut map: Vec<Vec<u32>> = Vec::new();

    build_map(file, map.as_mut());

    let mut visible_trees = 0;

    for i in 0..map.len() {
        if let Some(row) = map.get(i) {
            for j in 0..row.len() {
                if let Some(current_tree) = row.get(j) {
                    //check up
                    let mut visible = true;
                    for up in 0..i {
                        if let Some(prev_row) = map.get(up) {
                            if let Some(prev_tree) = prev_row.get(j) {
                                if current_tree <= prev_tree {
                                    visible = false;
                                    break;
                                }
                            }
                        }
                    }
                    if visible {
                        visible_trees += 1;
                        continue;
                    }
                    //check down
                    visible = true;
                    for down in i + 1..map.len() {
                        if let Some(next_row) = map.get(down) {
                            if let Some(next_tree) = next_row.get(j) {
                                if current_tree <= next_tree {
                                    visible = false;
                                    break;
                                }
                            }
                        }
                    }
                    if visible {
                        visible_trees += 1;
                        continue;
                    }
                    //check left
                    visible = true;
                    for left in 0..j {
                        if let Some(prev_tree) = row.get(left) {
                            if current_tree <= prev_tree {
                                visible = false;
                                break;
                            }
                        }
                    }
                    if visible {
                        visible_trees += 1;
                        continue;
                    }
                    //check right
                    visible = true;
                    for right in j + 1..row.len() {
                        if let Some(next_tree) = row.get(right) {
                            if current_tree <= next_tree {
                                visible = false;
                                break;
                            }
                        }
                    }
                    if visible {
                        visible_trees += 1;
                        continue;
                    }
                }
            }
        }
    }

    println!("{}", visible_trees);
}

fn build_map(file: File, map: &mut Vec<Vec<u32>>) {
    io::BufReader::new(file).lines().for_each(|line| {
        if let Ok(line) = line {
            let numbers: Vec<u32> = line
                .chars()
                .filter(|x| x.is_ascii_digit())
                .map(|x| x.to_digit(10).unwrap())
                .collect();
            map.push(numbers)
        }
    });
}

pub fn get_best_scenic_score() {
    let file = open_file(
        String::from_str("./inputs/day8.txt").expect("Not a string..."),
    );

    let mut map: Vec<Vec<u32>> = Vec::new();

    build_map(file, map.as_mut());

    let mut best_scenic_score = 0;

    for i in 0..map.len() {
        if let Some(row) = map.get(i) {
            for j in 0..row.len() {
                if let Some(current_tree) = row.get(j) {
                    //check up
                    let mut up_score = 0;
                    for up in (0..i).rev() {
                        if let Some(prev_row) = map.get(up) {
                            if let Some(prev_tree) = prev_row.get(j) {
                                up_score += 1;
                                if current_tree <= prev_tree {
                                    break;
                                }
                            }
                        }
                    }
                    //check down
                    let mut down_score = 0;
                    for down in i + 1..map.len() {
                        if let Some(next_row) = map.get(down) {
                            if let Some(next_tree) = next_row.get(j) {
                                down_score += 1;
                                if current_tree <= next_tree {
                                    break;
                                }
                            }
                        }
                    }
                    //check left
                    let mut left_score = 0;
                    for left in (0..j).rev() {
                        if let Some(prev_tree) = row.get(left) {
                            left_score += 1;
                            if current_tree <= prev_tree {
                                break;
                            }
                        }
                    }
                    //check right
                    let mut right_score = 0;
                    for right in j + 1..row.len() {
                        if let Some(next_tree) = row.get(right) {
                            right_score += 1;
                            if current_tree <= next_tree {
                                break;
                            }
                        }
                    }
                    let current_scenic_score =
                        up_score * down_score * left_score * right_score;
                    if best_scenic_score < current_scenic_score {
                        best_scenic_score = current_scenic_score;
                    }
                }
            }
        }
    }

    println!("{}", best_scenic_score);
}
