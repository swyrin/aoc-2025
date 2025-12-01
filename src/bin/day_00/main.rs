use std::{env, fs, path::MAIN_SEPARATOR};

use regex::Regex;

/// Get input file name based on the current running binary file.
///
/// For example, if running inside `src/bin/day_00/main.rs`,
/// the function returns `input/day_00_[mode].txt`.
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

#[forbid(unsafe_code)]
fn main() -> () {
    println!("Part 1: {}", part_1(false));
    println!("----------------");
    println!("Part 2: {}", part_2(false));
}

/// Remember to edit the test.
#[cfg(test)]
mod aoc_test {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(expected = { 4 })]
    fn result_part_1(expected: usize) {
        assert_eq!(part_1(true), expected)
    }

    #[parameterized(expected = { 8 })]
    fn result_part_2(expected: usize) {
        assert_eq!(part_2(true), expected)
    }
}

#[forbid(unsafe_code)]
fn part_1(is_sample: bool) -> usize {
    let path = get_input_path(is_sample);
    let _content = fs::read_to_string(path).expect("File read error.");

    4
}

#[forbid(unsafe_code)]
fn part_2(is_sample: bool) -> usize {
    let path = get_input_path(is_sample);
    let _content = fs::read_to_string(path).expect("File read error.");

    8
}
