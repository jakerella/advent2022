use std::fmt::format;

use regex::Regex;
use itertools::Itertools;


pub fn day5(input: String) {
    
    // let mut stacks: Vec<&str> = Vec::new();

    let crate_re = Regex::new(r"\s*\[([A-Z])\]\s*").unwrap();
    let instr_re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    let mut crates: Vec<Vec<String>> = Vec::new();
    
    #[derive(Debug)]
    struct Move {
        count: u8,
        from: usize,
        to: usize
    }
    
    let mut instructions: Vec<Move> = Vec::new();

    for line in input.lines() {
        let mut line_crates: Vec<String> = Vec::new();

        if line.starts_with("  ") || line.starts_with("[") {
            let padded = format!("{} ", line);
            
            for chunk in &padded.chars().chunks(4) {
                let crate_def = String::from_iter(chunk);
                let mut crate_id = String::from("");

                if crate_re.is_match(&crate_def.to_string()) {
                    let id_match = crate_re.captures_iter(&crate_def[..]).nth(0);
                    match id_match {
                        Some(id) => crate_id = String::from(&id[1]),
                        None => { println!("bad crate? {:?}", id_match); }
                    }
                }
                
                line_crates.push(crate_id);
            }

        } else if line.starts_with("move") {
            // println!("instr: {:?}", line);
            for cap in instr_re.captures_iter(&line) {
                let mut m = Move {
                    count: cap[1].parse().unwrap(),
                    from: cap[2].parse().unwrap(),
                    to: cap[3].parse().unwrap()
                };
                instructions.push(m);
            }
        }

        if line_crates.len() > 0 {
            crates.push(line_crates);
        }
    }

    let stack_count = crates[0].len();

    // println!("there are {} stacks: line crates: {:?}", stack_count, crates);

    let mut stacks:  Vec<Vec<String>> = vec![Vec::new(); stack_count];

    for row in crates {
        for stack_i in (0..stack_count) {
            if row[stack_i].len() > 0 {
                stacks[stack_i].push(String::from(&row[stack_i]));
            }
        }
    }

    // println!("instructions: {:?}", instructions);

    let mut part2_stacks = stacks.clone();

    // Part 1
    // println!("stacks before moving: {:?}", stacks);
    for step in &instructions {
        for i in (0..step.count) {
            let c = String::from(&stacks[step.from - 1][0]);
            stacks[step.to - 1].insert(0, c);
            stacks[step.from - 1].remove(0);
        }
    }
    // println!("stacks after moving: {:?}", stacks);

    let mut message = String::from("");
    for s in stacks {
        message.push(s[0].chars().next().unwrap());
    }

    println!("Part 1: answer: {:?}", &message);
    
    
    // Part 2
    // println!("stacks before moving: {:?}", part2_stacks);
    for step in &instructions {
        let mut m: Vec<String> = Vec::new();
        for i in (0..step.count) {
            m.push(String::from(&part2_stacks[step.from - 1][usize::from(i)]));
        }
        m = m.into_iter().rev().collect();
        for c in &m {
            part2_stacks[step.to - 1].insert(0, c.to_string());
            part2_stacks[step.from - 1].remove(0);
        }
    }
    // println!("stacks after moving: {:?}", part2_stacks);

    message = String::from("");
    for s in part2_stacks {
        message.push(s[0].chars().next().unwrap());
    }
    
    println!("Part 2: answer: {:?}", &message);
}
