
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
