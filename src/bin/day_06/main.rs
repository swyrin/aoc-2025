use std::{env, fs, path::MAIN_SEPARATOR};

use num_bigint::BigUint;
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
        .next_back()
        .expect("Seems like there isn't any slash?");

    // Linux does not have . in file name.
    let file_components: Vec<&str> = exe_name.split(".").collect();

    let mut bin_name = exe_name;

    if !file_components.is_empty() {
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
fn main() {
    println!("Part 1: {}", part_1(false));
    println!("Part 2: {}", part_2(false));
}

#[forbid(unsafe_code)]
fn part_1(is_sample: bool) -> BigUint {
    let path = get_input_path(is_sample);
    let content = fs::read_to_string(path).expect("File read error.");

    let mut total: BigUint = BigUint::ZERO;

    let mut lines: Vec<Vec<&str>> = vec![];

    for line in content.lines() {
        let line: Vec<&str> = line.split(" ").collect();

        let filtered_empty: Vec<&str> = line.into_iter().filter(|x| !x.is_empty()).collect();

        lines.push(filtered_empty);
    }

    let lc = lines.len();

    let operands = lines.last().expect("No last element?");
    let numbers: Vec<&Vec<&str>> = lines.iter().take(lc - 1).collect();

    let width = lines[0].len();
    let height = lc - 1;

    for w in 0..width {
        let mut val = if operands[w] == "*" {
            BigUint::from(1 as u32)
        } else {
            BigUint::ZERO
        };
        let is_mul = operands[w] == "*";

        for h in 0..height {
            let num = numbers[h][w].parse::<i32>().expect("NaN");
            let num = BigUint::from(num as u32);

            if is_mul {
                val *= num;
            } else {
                val += num;
            }
        }

        total += val;
    }

    total
}

#[forbid(unsafe_code)]
fn part_2(is_sample: bool) -> BigUint {
    let path = get_input_path(is_sample);
    let content = fs::read_to_string(path).expect("File read error.");

    let mut total: BigUint = BigUint::ZERO;

    let lines: Vec<&str> = content.lines().collect();

    let height = lines.len();
    let width = lines[0].len();

    let mut numbers: Vec<u32> = vec![];

    for w in (0..width).rev() {
        let number_count = height - 1;
        let mut parsed_num = 0;

        for h in 0..number_count {
            let character = lines[h].chars().nth(w).expect("Huh?");

            if character != ' ' {
                parsed_num = parsed_num * 10 + character.to_digit(10).expect("NaN");
            }
        }

        if parsed_num == 0 {
            numbers.clear();
            continue;
        }

        numbers.push(parsed_num);

        let operand = lines[height - 1]
            .chars()
            .nth(w)
            .expect("Did you turn off whitespace trim?");

        if operand != ' ' {
            let mut val = if operand == '*' {
                BigUint::from(1 as u32)
            } else {
                BigUint::ZERO
            };
            let is_mul = operand == '*';

            for number in numbers.iter() {
                if is_mul {
                    val *= BigUint::from(*number);
                } else {
                    val += BigUint::from(*number);
                }
            }

            total += val;
        }
    }

    total
}

/// Remember to edit the test.
#[cfg(test)]
mod aoc_test {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(expected = { 4277556 })]
    fn result_part_1(expected: usize) {
        assert_eq!(part_1(true), BigUint::from(expected))
    }

    #[parameterized(expected = { 3263827 })]
    fn result_part_2(expected: usize) {
        assert_eq!(part_2(true), BigUint::from(expected))
    }
}
