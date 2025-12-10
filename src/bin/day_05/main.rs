use std::{cmp, collections::HashSet, env, fs, path::MAIN_SEPARATOR};

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
fn part_1(is_sample: bool) -> usize {
    let path = get_input_path(is_sample);
    let content = fs::read_to_string(path).expect("File read error.");

    let mut reading_actual_input = false;
    let mut ranges: HashSet<(i64, i64)> = HashSet::new();
    let mut count = 0;

    for line in content.lines() {
        if line.is_empty() {
            reading_actual_input = true;
            continue;
        }

        if !reading_actual_input {
            let heads: Vec<&str> = line.split("-").collect();

            let head = str::parse::<i64>(heads[0]).expect("No head.");
            let tail = str::parse::<i64>(heads[1]).expect("No tail.");

            ranges.insert((head, tail));
        } else {
            let number = line.parse::<i64>().expect("Fuck you.");
            let mut has_match = false;

            for range in ranges.clone().into_iter() {
                let a = range.0;
                let b = range.1;

                if a <= number && number <= b {
                    has_match = true;
                    break;
                }
            }

            if has_match {
                count += 1;
            }
        }
    }

    count
}

#[forbid(unsafe_code)]
fn part_2(is_sample: bool) -> i64 {
    let path = get_input_path(is_sample);
    let content = fs::read_to_string(path).expect("File read error.");

    let mut ranges: Vec<Vec<i64>> = vec![];

    for line in content.lines() {
        if line.is_empty() {
            break;
        }

        let heads: Vec<&str> = line.split("-").collect();

        let head = str::parse::<i64>(heads[0]).expect("No head.");
        let tail = str::parse::<i64>(heads[1]).expect("No tail.");

        ranges.push(vec![head, tail]);
    }

    // It's been years since I last seen a range combinator
    ranges.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut combined: Vec<Vec<i64>> = vec![];
    combined.push(ranges[0].clone());

    for i in 1..ranges.len() {
        let current: Vec<i64> = ranges[i].clone();
        let j: usize = combined.len() - 1;

        if combined[j][0] <= current[0] && current[0] <= combined[j][1] {
            combined[j][1] = cmp::max(current[1], combined[j][1]);
        } else {
            combined.push(current);
        }
    }

    let mut total = 0;

    for r in combined {
        let a = r[0];
        let b = r[1];

        total += (b - a) + 1;
    }

    total
}

/// Remember to edit the test.
#[cfg(test)]
mod aoc_test {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(expected = { 3 })]
    fn result_part_1(expected: usize) {
        assert_eq!(part_1(true), expected)
    }

    #[parameterized(expected = { 14 })]
    fn result_part_2(expected: usize) {
        assert_eq!(part_2(true), expected as i64)
    }
}
