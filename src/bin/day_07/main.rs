use std::{
    collections::{HashMap, HashSet},
    env, fs,
    path::MAIN_SEPARATOR,
    time::Instant,
};

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
    let now = Instant::now();
    println!("Part 1: {}", part_1(false));
    let elapsed_time = now.elapsed();

    println!("Running part_1() took {} ms.", elapsed_time.as_millis());

    let now = Instant::now();
    println!("Part 2: {}", part_2(false));
    let elapsed_time = now.elapsed();

    println!("Running part_2() took {} ms.", elapsed_time.as_millis());
}

#[forbid(unsafe_code)]
fn part_1(is_sample: bool) -> BigUint {
    let path = get_input_path(is_sample);
    let content = fs::read_to_string(path).expect("File read error.");

    let mut require_x = HashSet::new();

    let lines = content.lines();

    for line in lines.clone().take(1) {
        for (x, chr) in line.chars().enumerate() {
            if chr == 'S' {
                require_x.insert(x);
                break;
            }
        }
    }

    let mut total = BigUint::ZERO;

    for line in lines.skip(2).step_by(2) {
        let mut processed_x = HashSet::new();
        let mut next_x_require = HashSet::new();

        for (x, chr) in line.chars().enumerate() {
            if chr == '^' && require_x.contains(&x) {
                processed_x.insert(x);
                total += BigUint::from(1_u32);

                next_x_require.insert(x - 1);
                next_x_require.insert(x + 1);
            }
        }

        for v in processed_x {
            require_x.remove(&v);
        }

        for v in next_x_require {
            require_x.insert(v);
        }
    }

    total
}

#[forbid(unsafe_code)]
fn part_2(is_sample: bool) -> BigUint {
    let path = get_input_path(is_sample);
    let content = fs::read_to_string(path).expect("File read error.");

    // how many rays to reach x
    let mut rays: HashMap<usize, BigUint> = HashMap::new();

    let lines = content.lines();

    for line in lines.clone().take(1) {
        for (x, chr) in line.chars().enumerate() {
            if chr == 'S' {
                rays.insert(x, BigUint::from(1_u32));
                break;
            }
        }
    }

    for line in lines.skip(2).step_by(2) {
        let mut next_rays = HashMap::new();

        for (x, chr) in line.chars().enumerate() {
            if chr == '^' {
                for (k, v) in &rays {
                    // branch if we see a spiltter.
                    if *k == x {
                        next_rays
                            .entry(x + 1)
                            .and_modify(|umeow| *umeow += v)
                            .or_insert(v.clone());

                        next_rays
                            .entry(x - 1)
                            .and_modify(|umeow| *umeow += v)
                            .or_insert(v.clone());
                    }
                }
            } else {
                for (k, v) in &rays {
                    if *k == x {
                        // or else, we go down.
                        next_rays
                            .entry(x)
                            .and_modify(|umeow| *umeow += v)
                            .or_insert(v.clone());
                    }
                }
            }
        }

        rays = next_rays;
    }

    rays.values().sum::<BigUint>()
}

/// Remember to edit the test.
#[cfg(test)]
mod aoc_test {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(expected = { 21 })]
    fn result_part_1(expected: usize) {
        assert_eq!(part_1(true), BigUint::from(expected))
    }

    #[parameterized(expected = { 40 })]
    fn result_part_2(expected: usize) {
        assert_eq!(part_2(true), BigUint::from(expected))
    }
}
