
use std::{collections::HashMap, ops::Index};

#[derive(Debug, Clone)]
struct Spot {
    loc: [usize; 2],
    neighbors: Vec<String>,
    is_end: bool,
    dist: u16
}

pub fn day12(input: String) {
    let heights = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut unvisited: HashMap<String, Spot> = HashMap::new();
    let mut visited: HashMap<String, Spot> = HashMap::new();
    let mut end: (usize, usize) = (0,0);
    let mut start: (usize, usize) = (0,0);

    for line in input.lines() {
        let mut row = String::from(line).chars().collect::<Vec<char>>();
        match row.iter().position(|&h| h == 'S') {
            Some(v) => {
                row[v] = 'a';
                start = (map.len(), v);
            },
            None => {}
        }
        match row.iter().position(|&h| h == 'E') {
            Some(v) => {
                row[v] = 'z';
                end = (map.len(), v);
            },
            None => {}
        }
        map.push(Vec::from(row));
    }

    // println!("Map: {:?}", map);
    println!("Going from {:?} to {:?}", start, end);

    for y in (0..map.len()) {
        for x in (0..map[0].len()) {
            let h_val = heights.iter().position(|&v| v == map[y][x]).unwrap();

            let mut neighbors = Vec::new();

            if is_neighbor(&map, h_val, (y as i16, (x+1) as i16)) {  // look right
                neighbors.push(format!("{},{}", y, x+1));
            }
            if is_neighbor(&map, h_val, (y as i16, (x as i16)-1)) {  // look left
                neighbors.push(format!("{},{}", y, x-1));
            }
            if is_neighbor(&map, h_val, ((y as i16)-1, x as i16)) {  // look up
                neighbors.push(format!("{},{}", y-1, x));
            }
            if is_neighbor(&map, h_val, ((y+1) as i16, x as i16)) {  // look down
                neighbors.push(format!("{},{}", y+1, x));
            }

            let dist: u16 = if y==start.0 && x==start.1 { 0 } else { 9998 };
            let s = Spot {
                loc: [y, x],
                neighbors,
                dist,
                is_end: y == end.0 && x == end.1
            };
            unvisited.insert(format!("{},{}", y, x), s);
        }
    }

    // println!("Unvisited: {:?}", unvisited["0,0"]);
    // println!("Unvisited: {:?}", unvisited["20,0"]);

    let mut curr_spot = unvisited[&format!("{},{}", start.0, start.1)].clone();
    while unvisited.len() > 0 {
        curr_spot = unvisited[&get_min_dist(&unvisited)].clone();
        // println!("Now processing spot: {:?} with dist {:?}", curr_spot.loc, curr_spot.dist);

        for n_id in &curr_spot.neighbors {
            if unvisited.contains_key(n_id) {
                let id = String::from(n_id);
                if unvisited[&id].dist > (curr_spot.dist + 1) {
                    unvisited.entry(id).and_modify(|s| {
                        s.dist = curr_spot.dist + 1;
                        // println!("Modified neighbor {:?} with new dist {:?}", s.loc, s.dist);
                    });
                }
            }
        }
        unvisited.remove(&format!("{},{}", curr_spot.loc[0], curr_spot.loc[1]));
        if curr_spot.loc[0] == end.0 && curr_spot.loc[1] == end.1 {
            println!("Part 1: Reached the end with min dist {:?}", curr_spot.dist);
            break;
        }
    }

}

fn is_neighbor(map: &Vec<Vec<char>>, h: usize, pos: (i16, i16)) -> bool {
    let heights = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    if pos.0 >= 0 && pos.0 < (map.len() as i16) && pos.1 >= 0 && pos.1 < (map[0].len() as i16) {
        let n_val: i16 = heights.iter().position(|&v| v == map[pos.0 as usize][pos.1 as usize]).unwrap() as i16;
        if (h as i16) >= (n_val - 1) {
            return true;
        }
    }
    return false;
}

fn get_min_dist(list: &HashMap<String, Spot>) -> String {
    let mut hold_dist: u16 = 9999;
    let mut hold_id: String = String::from("0,0");
    for (k, s) in list {
        if list[k].dist < hold_dist {
            hold_dist = list[k].dist;
            hold_id = format!("{},{}", list[k].loc[0], list[k].loc[1]);
        }
    }
    return hold_id;
}
