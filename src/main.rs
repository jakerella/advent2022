
#![allow(unused)]
use std::{ptr::null, collections::HashMap};
use std::path::PathBuf;
use std::path::Path;
use clap::Parser;

use crate::advent::*;

pub mod advent;

#[derive(Parser)]
struct Args {
    /// What Advent day would you like to run?
    #[arg(short = 'd', long = "day")]
    day: String,

    /// (Optional) What data input file would you like to use?
    #[arg(short = 'f', long = "input-file", value_name = "INPUT-FILE")]
    input: Option<PathBuf>,
}

fn main() {
    let cli = Args::parse();
    println!("\n*** Running Advent Day {:?} ***\n", cli.day);

    let mut methods: HashMap<String, fn(String)> = HashMap::new();


    // @TODO: add each day in here
    methods.insert("1".to_string(), advent::day1::day1);
    methods.insert("2".to_string(), advent::day2::day2);
    methods.insert("3".to_string(), advent::day3::day3);
    methods.insert("4".to_string(), advent::day4::day4);
    methods.insert("5".to_string(), advent::day5::day5);
    methods.insert("6".to_string(), advent::day6::day6);
    methods.insert("7".to_string(), advent::day7::day7);
    methods.insert("8".to_string(), advent::day8::day8);
    methods.insert("9".to_string(), advent::day9::day9);
    methods.insert("10".to_string(), advent::day10::day10);
    methods.insert("11".to_string(), advent::day11::day11);
    methods.insert("12".to_string(), advent::day12::day12);
    methods.insert("13".to_string(), advent::day13::day13);
    methods.insert("14".to_string(), advent::day14::day14);
    methods.insert("15".to_string(), advent::day15::day15);
    // day 16
    methods.insert("17".to_string(), advent::day17::day17);


    let day_input_path = format!("{}{}{}", "./src/inputs/day", cli.day, ".txt");
    let mut input: String = "".to_string();
    if let Some(path) = cli.input.as_deref() {
        println!("Reading input from: {:?}", path.to_str());
        input = std::fs::read_to_string(path).expect("Could not read input file");
        // println!("{:?}", input)
    } else if Path::new(&day_input_path).exists() {
        println!("Reading input from: {:?}", day_input_path);
        input = std::fs::read_to_string(day_input_path).expect("Could not read input file");
    }

    let valid = match methods.get(&cli.day) {
        Some(f) => f(input),
        None => println!("INVALID DAY"),
    };
}


// fn process(filepath: Option<std::path::PathBuf>) {
//     // let input: String = null()
//     if let Some(path) = filepath.as_deref() {
//         println!("Reading input from: {:?}", path.to_str());
//         let content = std::fs::read_to_string(path).expect("Could not read input file");
//         for line in content.lines() {
//             println!("{}", line);
//         }
//     }
// }
