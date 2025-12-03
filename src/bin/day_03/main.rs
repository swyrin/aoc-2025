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
    println!("----------------");
    println!("Part 2: {}", part_2(false));
}

#[forbid(unsafe_code)]
fn part_1(is_sample: bool) -> usize {
    let path = get_input_path(is_sample);
    let content = fs::read_to_string(path).expect("File read error.");
    let mut total: usize = 0;

    for line in content.lines() {
        let numbers: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();

        // basically, a linear function mx + b, at its maxima when m and b reaches max.

        let index_max_1: usize = numbers
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
            .expect("Hmmmm.");

        let max_1 = numbers[index_max_1];

        let mut max_left = u32::MIN;
        let mut max_right = u32::MIN;

        for left_i in numbers.iter().take(index_max_1) {
            max_left = max_left.max(*left_i);
        }

        for right_i in numbers.iter().skip(index_max_1 + 1) {
            max_right = max_right.max(*right_i);
        }

        let mut result1 = max_left * 10 + max_1;
        let mut result2 = max_1 * 10 + max_right;

        if max_left == 0 {
            result1 = 0;
        }

        if max_right == 0 {
            result2 = 0;
        }

        let volt = u32::max(result1, result2);

        total += volt as usize;
    }

    total
}

/// NSFW warning?
#[derive(Debug, Copy, Clone)]
struct Umipai {
    digit: u32,
}

#[forbid(unsafe_code)]
fn part_2(is_sample: bool) -> i128 {
    let path = get_input_path(is_sample);
    let content = fs::read_to_string(path).expect("File read error.");

    // can't believe we jumped from 2 to 12, smh.
    let mut total: i128 = 0;

    // basically, try to form the longest number chain possible,
    // like 9999..9, 999..8, ... and so on.
    // https://en.wikipedia.org/wiki/Tournament_sort
    // (sans the min-heap part).
    for line in content.lines() {
        let numbers: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        let mut arr: Vec<Umipai> = vec![];

        for (i, v) in numbers.iter().enumerate() {
            let num = *v;
            let obj = Umipai { digit: num };

            loop {
                // for some reasons, leave this code above the `loop` doesn't work.
                if arr.is_empty() {
                    arr.push(obj);
                    break;
                    // continue;
                }

                let unused_number_count = line.len() - i;
                let slot_count = 12 - arr.len();
                let last = *arr.last().unwrap();

                // basically, clean up small numbers so that
                // the larger number (champion) will join the inner bracket.
                // to create a chain of 9's, then 8's, ...
                if last.digit < num && unused_number_count > slot_count {
                    arr.pop();
                    continue;
                }

                // then the champion joins the bracket.
                if slot_count > 0 {
                    arr.push(Umipai { digit: num });
                    break;
                } else {
                    break;
                }
            }
        }

        if arr.is_empty() {
            panic!("Why are you panic?")
        }

        let number: i128 = arr
            .iter()
            .fold(0, |umi, meow| umi * 10 + meow.digit as i128);

        total += number;
    }

    total
}

/// Remember to edit the test.
#[cfg(test)]
mod aoc_test {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(expected = { 357 })]
    fn result_part_1(expected: usize) {
        assert_eq!(part_1(true), expected)
    }

    #[parameterized(expected = { 3121910778619 })]
    fn result_part_2(expected: usize) {
        assert_eq!(part_2(true), expected as i128)
    }
}
