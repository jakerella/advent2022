
pub fn day17(input: String) {

    let motions = input.chars().collect::<Vec<char>>();
    let rocks = get_rocks();
    // println!("rock types:\n{:?}", rocks);

    let mut room: Vec<[char; 7]> = Vec::from([['.'; 7], ['.'; 7], ['.'; 7], ['.'; 7]]);
    let mut max_rocks: u64 = 2022;  // Part 2: 1000000000000
    let mut max_height: u64 = 0;
    let mut curr_motion: usize = 0;
    let mut curr_rock: usize = 0;

    for i in (0..max_rocks) {
        // println!("rock #{:?} dropping: {:?}", i+1, rocks[curr_rock]);
        let mut loc: (u8, u64) = (2, max_height + 3);

        if room.len() <= (max_height as usize + rocks[curr_rock].len() + 3) {
            let add_rows = (max_height + 3) as i64 - (room.len() as i64) + (rocks[curr_rock].len() as i64);
            // println!("raising the roof by {} rows", add_rows);
            for j in (0..add_rows) {
                room.push(['.'; 7]);
            }
        }
        
        loop {
            jet_blast(&mut loc, &room, &rocks[curr_rock], motions[curr_motion]);
            curr_motion += 1;
            if (curr_motion >= motions.len()) { curr_motion = 0; }
            
            if !move_down(&mut loc, &room, &rocks[curr_rock]) {
                // println!("can't move rock down, will be placed at {:?}", loc);
                for (row_i, row) in rocks[curr_rock].iter().enumerate() {
                    for (block_i, block) in row.iter().enumerate() {
                        if block == &'#' {
                            room[(loc.1 as usize) + row_i][(loc.0 as usize) + block_i] = '#';
                        }
                    }
                }

                'room_rows: for row_i in (0..room.len()) {
                    for c in room[room.len() - row_i - 1] {
                        if c == '#' {
                            max_height = (room.len() - row_i) as u64;
                            break 'room_rows;
                        }
                    }
                }

                break;
            }
        }

        // println!("room after {} rocks:", i+1);
        // for i in (0..room.len()) {
        //     println!("{:?}", String::from_iter(room[room.len()-i-1]));
        // }

        curr_rock += 1;
        if (curr_rock >= rocks.len()) { curr_rock = 0; }
    }

    println!("Part 1 answer, max height: {:?}", max_height);

}

fn jet_blast(loc: &mut (u8, u64), room: &Vec<[char; 7]>, rock: &Vec<Vec<char>>, motion: char) {
    for (i, row) in rock.iter().enumerate() {
        // println!("jet blast {}: checking rock row {:?} against room row {:?}", motion, row, room[loc.1 as usize]);
        if motion == '>' {
            let right_edge = get_right_index(row);
            if (loc.0 + right_edge) + 1 >= (room[0].len() as u8) {
                // println!("hit right wall");
                return;
            } else if room[loc.1 as usize + i][(loc.0 + right_edge) as usize + 1] == '#' {
                // println!("hit the tower moving right");
                return;
            }
        } else {
            let left_edge = get_left_index(row);
            if ((loc.0 + left_edge) as i8) - 1 < 0 {
                // println!("hit left wall");
                return;
            } else if room[loc.1 as usize + i][(loc.0 + left_edge) as usize - 1] == '#' {
                // println!("hit the tower moving left");
                return;
            }
        }
    }

    // println!("moving {:?} to the {} from {:?}", rock, motion, loc);
    if motion == '>' { loc.0 += 1 } else { loc.0 -= 1 }
}

fn get_right_index(row: &Vec<char>) -> u8 {
    for i in (0..row.len()) {
        if row[row.len()-i-1] == '#' { return ((row.len()-i-1) as u8); }
    }
    return 0;
}
fn get_left_index(row: &Vec<char>) -> u8 {
    for i in (0..row.len()) {
        if row[i] == '#' { return (i as u8); }
    }
    return 0;
}

fn move_down(loc: &mut (u8, u64), room: &Vec<[char; 7]>, rock: &Vec<Vec<char>>) -> bool {
    if (loc.1 as i64) - 1 < 0 { return false; }

    for (rock_row_i, rock_row) in rock.iter().enumerate() {
        for (rock_col_i, rock_col) in rock_row.iter().enumerate() {
            if rock_col == &'#' && room[loc.1 as usize + rock_row_i - 1][loc.0 as usize + rock_col_i] == '#' {
                // println!("something right below rock on room row");
                return false;
            }
        }
    }

    // println!("moving {:?} DOWN from {:?}", rock, loc);
    loc.1 -= 1;
    return true;  // false indicates we CAN'T move down
}


fn get_rocks() -> Vec<Vec<Vec<char>>> {
    return Vec::from([
        Vec::from([
            Vec::from(['#', '#', '#', '#'])
        ]),

        Vec::from([
            Vec::from(['.', '#', '.']),
            Vec::from(['#', '#', '#']),
            Vec::from(['.', '#', '.'])
        ]),

        // Note that this one is "flipped" vertically to match room rows (0 at bottom)
        Vec::from([
            Vec::from(['#', '#', '#']),
            Vec::from(['.', '.', '#']),
            Vec::from(['.', '.', '#']),
        ]),

        Vec::from([
            Vec::from(['#']),
            Vec::from(['#']),
            Vec::from(['#']),
            Vec::from(['#'])
        ]),

        Vec::from([
            Vec::from(['#', '#']),
            Vec::from(['#', '#'])
        ])
    ]);
}
