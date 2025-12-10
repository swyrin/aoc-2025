use std::{env, fs, path::MAIN_SEPARATOR, time::Instant};

use petgraph::algo;
use petgraph::graph::UnGraph;
use regex::Regex;
use std::collections::BTreeMap;

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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Point3 {
    x: isize,
    y: isize,
    z: isize,
}

impl Point3 {
    fn distance_from(&self, other: &Self) -> isize {
        (self.x - other.x) * (self.x - other.x)
            + (self.y - other.y) * (self.y - other.y)
            + (self.z - other.z) * (self.z - other.z)
    }

    fn from_str(line: &str) -> Self {
        let numbers: Vec<&str> = line.trim().split(",").take(3).collect();

        Self {
            x: numbers[0].parse().unwrap(),
            y: numbers[1].parse().unwrap(),
            z: numbers[2].parse().unwrap(),
        }
    }
}

#[forbid(unsafe_code)]
fn part_1(is_sample: bool) -> usize {
    let path = get_input_path(is_sample);
    let content = fs::read_to_string(path).expect("File read error.");

    let points: Vec<Point3> = content.lines().map(Point3::from_str).collect();

    let mut edges = BTreeMap::<isize, (u32, u32)>::new();

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let a = &points[i];
            let b = &points[j];
            let d = a.distance_from(b);
            edges.insert(d, (i as u32, j as u32));
        }
    }

    let graph =
        UnGraph::<u32, ()>::from_edges(edges.values().take(if is_sample { 10 } else { 1000 }));

    let mut sccs: Vec<usize> = petgraph::algo::kosaraju_scc(&graph)
        .iter()
        .map(|x| x.len())
        .collect();

    sccs.sort_unstable();

    sccs.iter().rev().take(3).product()
}

#[forbid(unsafe_code)]
fn part_2(is_sample: bool) -> usize {
    let path = get_input_path(is_sample);
    let content = fs::read_to_string(path).expect("File read error.");

    let points: Vec<Point3> = content.lines().map(Point3::from_str).collect();
    let mut graph = UnGraph::<u32, ()>::new_undirected();

    let mut edges = BTreeMap::<isize, (u32, u32)>::new();

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let a = &points[i];
            let b = &points[j];
            let d = a.distance_from(b);
            edges.insert(d, (i as u32, j as u32));
        }
    }

    for i in 0..points.len() {
        graph.add_node(i as u32);
    }

    loop {
        let e = edges.pop_first().unwrap().1;
        graph.add_edge(e.0.into(), e.1.into(), ());

        if algo::connected_components(&graph) == 1 {
            let a = points[e.0 as usize].x;
            let b = points[e.1 as usize].x;
            return (b * a) as usize;
        }
    }
}

/// Remember to edit the test.
#[cfg(test)]
mod aoc_test {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(expected = { 40 })]
    fn result_part_1(expected: usize) {
        assert_eq!(part_1(true), expected)
    }

    #[parameterized(expected = { 25272 })]
    fn result_part_2(expected: usize) {
        assert_eq!(part_2(true), expected)
    }
}
