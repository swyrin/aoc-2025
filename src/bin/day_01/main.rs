use std::{env, fs, path::MAIN_SEPARATOR};

use regex::Regex;

/// Get input file name based on the current running binary file.
///
/// For example, if running inside `src/bin/day_0/main.rs`,
/// the function returns `input/day_0.txt`.
///
/// It's best to fold this code, btw.
fn get_input_path(is_sample: bool) -> String {
    // cargo test creates hash value in the resulting executable file.
    let hash_regex = Regex::new("-[a-zA-Z0-9]+").unwrap();

    let path = match env::current_exe() {
        Ok(path) => String::from(path.to_str().expect("Expect a string?")),
        Err(_e) => panic!("env::current_exe() failure!"),
    };

    let exe_name = path
        .split(MAIN_SEPARATOR)
        .last()
        .expect("Seems like there isn't any slash?");

    // Linux does not have . in file name.
    let file_components: Vec<&str> = exe_name.split(".").collect();

    let mut bin_name = exe_name;

    if file_components.len() > 0 {
        bin_name = file_components[0];
    }

    // clean up hash value, hopefully.
    let bin_name = hash_regex.replace_all(bin_name, "");

    let mut location = "input/".to_owned();
    location.push_str(&bin_name);

    if is_sample {
        location.push_str("_sample");
    } else {
        location.push_str("_personal");
    }

    location.push_str(".txt");

    location
}

/// Function for running part 1 code.
/// Most of AoC problems use uint as output.
///
/// Parse input yourself.
#[forbid(unsafe_code)]
fn func_part_1(is_sample: bool) -> usize {
    let path = get_input_path(is_sample);
    let content = fs::read_to_string(path).expect("File read error.");

    let lines = content.lines();
    let mut pos = 50;
    let mut count = 0;

    for line in lines {
        let direction = line.chars().nth(0).unwrap();
        let amount = &line[1..];

        let amount = amount.parse::<i32>().unwrap();

        match direction {
            'L' => {
                pos = (pos - amount) % 100;
            }
            'R' => {
                pos = (pos + amount) % 100;
            }
            _ => {
                panic!("Not a valid direction.")
            }
        }

        if pos == 0 {
            count += 1;
        }
    }

    count
}

/// Function for running part 2 code.
/// Most of AoC problems use uint as output.
///
/// Parse input yourself.
#[forbid(unsafe_code)]
fn func_part_2(is_sample: bool) -> i64 {
    let path = get_input_path(is_sample);
    let content = fs::read_to_string(path).expect("File read error.");

    let lines = content.lines();
    let mut pos = 50;
    let mut count = 0;

    for line in lines {
        let direction = line.chars().nth(0).unwrap();
        let amount = &line[1..];

        let amount = amount.parse::<i64>().unwrap();

        count += amount / 100;
        let amount = amount % 100;

        match direction {
            'L' => {
                if pos != 0 && pos - amount <= 0 {
                    count += 1;
                }

                pos = (pos - amount).rem_euclid(100);
            }
            'R' => {
                if pos != 0 && pos + amount >= 100 {
                    count += 1;
                }

                pos = (pos + amount).rem_euclid(100);
            }
            _ => {
                panic!("Not a valid direction.")
            }
        }
    }

    count
}

/// Main function, usually should be left as-is.
fn main() {
    println!("Part 1: {}", func_part_1(false));
    println!("Part 2: {}", func_part_2(false));
}

/// Testing module.
/// Only test against sample input/output.
#[cfg(test)]
mod aoc_test {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(expected = { 3 })]
    fn result_part_1(expected: usize) {
        assert_eq!(func_part_1(true), expected)
    }

    #[parameterized(expected = { 6 })]
    fn result_part_2(expected: usize) {
        assert_eq!(func_part_2(true), expected as i64)
    }
}
