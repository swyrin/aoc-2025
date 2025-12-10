use std::{env, fs, path::MAIN_SEPARATOR, time::Instant};

use geo::{Contains, LineString, Point, Polygon, Rect, point};
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
fn part_1(is_sample: bool) -> i128 {
    let path = get_input_path(is_sample);
    let content = fs::read_to_string(path).expect("File read error.");

    let mut points = vec![];

    for line in content.lines() {
        let items: Vec<&str> = line.split(",").collect();
        let x: i128 = items[0].parse().expect("NaN");
        let y: i128 = items[1].parse().expect("NaN");
        points.push((x, y));
    }

    let n = points.len();

    let mut max_area = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];

            let diff_x = (x2).abs_diff(x1) as i128 + 1_i128;
            let diff_y = (y2).abs_diff(y1) as i128 + 1_i128;

            max_area = max_area.max(diff_x * diff_y);
        }
    }

    max_area
}

#[forbid(unsafe_code)]
fn part_2(is_sample: bool) -> f64 {
    let path = get_input_path(is_sample);
    let content = fs::read_to_string(path).expect("File read error.");

    let mut points: Vec<Point> = vec![];

    for line in content.lines() {
        let items: Vec<&str> = line.split(",").collect();
        let x: i128 = items[0].parse().expect("NaN");
        let y: i128 = items[1].parse().expect("NaN");
        points.push(point! { x: x as f64, y: y as f64 });
    }

    let poly = Polygon::new(LineString::from(points.clone()), vec![]);

    let n = points.len();

    let mut max_area: f64 = 0_f64;

    for i in 0..n {
        for j in (i + 1)..n {
            let rect = Rect::new(points[i], points[j]);

            let (x1, y1) = points[i].x_y();
            let (x2, y2) = points[j].x_y();

            let diff_x = (x2 - x1).abs() + 1_f64;
            let diff_y = (y2 - y1).abs() + 1_f64;

            let area = diff_x * diff_y;

            if area > max_area && poly.contains(&rect) {
                max_area = max_area.max(area);
            }
        }
    }

    max_area
}

/// Remember to edit the test.
#[cfg(test)]
mod aoc_test {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(expected = { 50 })]
    fn result_part_1(expected: i128) {
        assert_eq!(part_1(true), expected)
    }

    #[parameterized(expected = { 24_f64 })]
    fn result_part_2(expected: f64) {
        assert_eq!(part_2(true), expected)
    }
}
