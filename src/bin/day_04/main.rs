use std::{collections::HashSet, env, fs, path::MAIN_SEPARATOR};

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

    let mut grid: Vec<Vec<char>> = vec![];

    for line in content.lines() {
        let x: Vec<_> = line.chars().collect();
        grid.push(x);
    }

    let mut paper_rolls = 0;

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let is_a_paper_roll = |r: i32, c: i32| {
        0 <= r && r < rows && 0 <= c && c < cols && grid[r as usize][c as usize] == '@'
    };

    for i in 0..rows {
        for j in 0..cols {
            let mut count = 0;

            if grid[i as usize][j as usize] == '.' {
                continue;
            }

            // 12h
            if is_a_paper_roll(i - 1, j) {
                count += 1;
            }

            // 1h30
            if is_a_paper_roll(i - 1, j + 1) {
                count += 1;
            }

            // 3h
            if is_a_paper_roll(i, j + 1) {
                count += 1;
            }

            // 4h30
            if is_a_paper_roll(i + 1, j + 1) {
                count += 1;
            }

            // 6h
            if is_a_paper_roll(i + 1, j) {
                count += 1;
            }

            // 7h30
            if is_a_paper_roll(i + 1, j - 1) {
                count += 1;
            }

            // 9h
            if is_a_paper_roll(i, j - 1) {
                count += 1;
            }

            // 10h30
            if is_a_paper_roll(i - 1, j - 1) {
                count += 1;
            }

            if count < 4 {
                paper_rolls += 1
            }
        }
    }

    paper_rolls
}

#[forbid(unsafe_code)]
fn part_2(is_sample: bool) -> usize {
    let path = get_input_path(is_sample);
    let content = fs::read_to_string(path).expect("File read error.");

    let mut grid: Vec<Vec<char>> = vec![];
    let mut ignore_list: HashSet<(i32, i32)> = HashSet::new();

    for line in content.lines() {
        let x: Vec<_> = line.chars().collect();
        grid.push(x);
    }

    let mut destroy_count = 0;

    loop {
        let mut paper_rolls = 0;

        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;

        let is_a_paper_roll = |v: &Vec<Vec<char>>, i: &HashSet<(i32, i32)>, r: i32, c: i32| {
            0 <= r
                && r < rows
                && 0 <= c
                && c < cols
                && !i.contains(&(r, c))
                && v[r as usize][c as usize] == '@'
        };

        for i in 0..rows {
            for j in 0..cols {
                let mut count = 0;

                if ignore_list.contains(&(i, j)) {
                    continue;
                }

                if grid[i as usize][j as usize] == '.' {
                    ignore_list.insert((i, j));
                    continue;
                }

                // 12h
                if is_a_paper_roll(&grid, &ignore_list, i - 1, j) {
                    count += 1;
                }

                // 1h30
                if is_a_paper_roll(&grid, &ignore_list, i - 1, j + 1) {
                    count += 1;
                }

                // 3h
                if is_a_paper_roll(&grid, &ignore_list, i, j + 1) {
                    count += 1;
                }

                // 4h30
                if is_a_paper_roll(&grid, &ignore_list, i + 1, j + 1) {
                    count += 1;
                }

                // 6h
                if is_a_paper_roll(&grid, &ignore_list, i + 1, j) {
                    count += 1;
                }

                // 7h30
                if is_a_paper_roll(&grid, &ignore_list, i + 1, j - 1) {
                    count += 1;
                }

                // 9h
                if is_a_paper_roll(&grid, &ignore_list, i, j - 1) {
                    count += 1;
                }

                // 10h30
                if is_a_paper_roll(&grid, &ignore_list, i - 1, j - 1) {
                    count += 1;
                }

                if count < 4 {
                    paper_rolls += 1;
                    ignore_list.insert((i, j));
                }
            }
        }

        if paper_rolls == 0 {
            break;
        } else {
            destroy_count += paper_rolls;
        }
    }

    destroy_count
}

/// Remember to edit the test.
#[cfg(test)]
mod aoc_test {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(expected = { 13 })]
    fn result_part_1(expected: usize) {
        assert_eq!(part_1(true), expected)
    }

    #[parameterized(expected = { 43 })]
    fn result_part_2(expected: usize) {
        assert_eq!(part_2(true), expected)
    }
}
