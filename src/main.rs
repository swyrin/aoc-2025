use std::{
    fs::{self, File},
    io::Write,
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The day number.
    #[arg(long, default_value_t = 1)]
    day: u8,
}

fn main() {
    let args = Args::parse();
    let day_number = args.day;

    let day_number_leftpad = format!("{:0>2}", day_number);
    let day_number_leftpad = day_number_leftpad.as_str();

    let input_base_dir = String::from("input/day_");
    let code_base_dir = String::from("src/bin/day_");

    // Input files.

    println!("Creating input files.");

    let _ = fs::create_dir("input");

    File::create_new(input_base_dir.clone() + day_number_leftpad + "_sample.txt")
        .expect("Sample input file already exists!");

    File::create_new(input_base_dir.clone() + day_number_leftpad + "_personal.txt")
        .expect("Personal input file already exists!");

    // Code file.

    println!("Creating code file.");

    let content = fs::read_to_string(code_base_dir.clone() + "00/main.rs")
        .expect("Unable to read the sample code.");

    fs::create_dir(code_base_dir.clone() + day_number_leftpad)
        .expect("Unable to create code directory.");

    let mut file = File::create_new(code_base_dir.clone() + day_number_leftpad + "/main.rs")
        .expect("Unable to create file.");

    file.write_all(content.as_bytes())
        .expect("Code file write error!");
}
