
use std::collections::HashMap;

pub fn day8(input: String) {
    
    let mut forest: Vec<Vec<u8>> = Vec::new();
    
    for line in input.lines() {
        let mut row: Vec<u8> = Vec::new();
        for t in String::from(line).chars() {
            row.push(t.to_string().parse::<u8>().unwrap());
        }
        forest.push(row);
    }

    let mut unblocked: u16 = 0;
    for y in (0..forest.len()) {
        for x in (0..forest[y].len()) {
            if is_visible(&forest, y, x) {
                unblocked += 1;
                // println!("{:?}, {:?} is NOT blocked", y, x);
            // } else {
            //     println!("{:?}, {:?} is blocked", y, x);
            }
        }
    }

    println!("Part 1answer {:?}", unblocked);
}

fn is_visible(forest: &Vec<Vec<u8>>, y:usize, x:usize) -> bool {
    if y == 0 || x == 0 || y == forest.len()-1 || x == forest[0].len()-1 {
        return true;
    }

    let h = forest[y][x];
    let mut left_vis = true;
    let mut right_vis = true;
    let mut up_vis = true;
    let mut down_vis = true;

    for left in (0..x) { if forest[y][left] >= h { left_vis = false; } }
    for right in (x+1..forest[0].len()) { if forest[y][right] >= h { right_vis = false; } }
    for up in (0..y) { if forest[up][x] >= h { up_vis = false; } }
    for down in (y+1..forest.len()) { if forest[down][x] >= h { down_vis = false; } }

    return left_vis || right_vis || up_vis || down_vis;
}
