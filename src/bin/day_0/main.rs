use std::{env, fs};

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
        .split("\\")
        .last()
        .expect("Seems like there isn't any slash?");

    let bin_name = exe_name
        .split(".")
        .nth(0)
        .expect("File seems like a dotfile.");

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
    let _content = fs::read_to_string(path).expect("File read error.");

    4
}

/// Function for running part 2 code.
/// Most of AoC problems use uint as output.
///
/// Parse input yourself.
#[forbid(unsafe_code)]
fn func_part_2(is_sample: bool) -> usize {
    let path = get_input_path(is_sample);
    let _content = fs::read_to_string(path).expect("File read error.");

    8
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

    #[parameterized(expected = { 4 })]
    fn result_part_1(expected: usize) {
        assert_eq!(func_part_1(true), expected)
    }

    #[parameterized(expected = { 8 })]
    fn result_part_2(expected: usize) {
        assert_eq!(func_part_2(true), expected)
    }
}
