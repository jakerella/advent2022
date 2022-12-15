
use std::collections::HashSet;
use std::cmp::{min,max};

pub fn day14(input: String) {
    
    let formations: Vec<Vec<(u32,u32)>> = input
        .split("\n").collect::<Vec<&str>>()
        .iter().map(|line| {
            return line.split(" -> ").collect::<Vec<&str>>()
                .iter().map(|rock| {
                    let pieces = rock.split(",").collect::<Vec<&str>>();
                    return (pieces[0].parse::<u32>().unwrap(), pieces[1].parse::<u32>().unwrap());
                })
                .collect::<Vec<(u32,u32)>>()
        }).collect::<Vec<Vec<(u32,u32)>>>();
    // println!("rock formation {:?}", formations);

    let mut blocked: HashSet<String> = HashSet::new();
    let mut lowest_rock = 0;

    for formation in formations {
        for i in (1..formation.len()) {
            if formation[i].0 == formation[i-1].0 {  // vertical
                for y in (min(formation[i-1].1, formation[i].1)..(max(formation[i-1].1, formation[i].1)+1)) {
                    if y > lowest_rock { lowest_rock = y }
                    blocked.insert(format!("{},{}", formation[i].0, y));
                }
            } else {  // horizontal
                for x in (min(formation[i-1].0, formation[i].0)..(max(formation[i-1].0, formation[i].0)+1)) {
                    if formation[i].1 > lowest_rock { lowest_rock = formation[i].1 }
                    blocked.insert(format!("{},{}", x, formation[i].1));
                }
            }
        }
    }
    // println!("blocked locations {:?}", blocked);
    // println!("lowest rock: {:?}", lowest_rock);

    // let mut sand_loc: Vec<(u32,u32)> = Vec::new();
    let mut sand_settled = 0;

    // Part 2
    let floor = lowest_rock + 2;

    while sand_settled < 50000 {  // infinite loop protection
        // new piece of sand
        let mut pos: (u32, u32) = (500,0);

        // if we ever get past the lowest rock it's going down forever
        // while pos.1 < lowest_rock {  // Part 1
        while pos.1 < floor {  // Part 2

            if pos.1 < (floor-1) && !blocked.contains(&format!("{},{}", pos.0, pos.1+1)) {
                pos.1 += 1;
                // println!("moving sand down to {:?}", pos);

            } else if pos.1 < (floor-1) && !blocked.contains(&format!("{},{}", pos.0-1, pos.1+1)) {
                pos.0 -= 1;
                pos.1 += 1;
                // println!("moving sand down-and-left to {:?}", pos);

            } else if pos.1 < (floor-1) && !blocked.contains(&format!("{},{}", pos.0+1, pos.1+1)) {
                pos.0 += 1;
                pos.1 += 1;
                // println!("moving sand down-and-right to {:?}", pos);

            } else {
                // println!("sand settled at {:?}", pos);
                // sand_loc.push(pos);
                blocked.insert(format!("{},{}", pos.0, pos.1));
                break;
            }
            
        }

        // if pos.1 >= lowest_rock { break; }  // Part 1
        sand_settled += 1;
        if pos.1 <= 0 { break; }  // Part 2

    }
    // println!("sand locations {:?}", sand_loc);
    println!("Total sand settled: {:?}", sand_settled);

    
}
