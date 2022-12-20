
use std::num;
use std::cmp::{min,max};
use std::collections::{HashMap, HashSet};
use regex::Regex;

pub fn day15(input: String) {
    
    let coord_re = Regex::new(r"Sensor at x=([0-9\-]+), y=([0-9\-]+).+?x=([0-9\-]+), y=([0-9\-]+)").unwrap();

    let mut data: Vec<(i32, i32, i32, i32)> = Vec::new();

    for line in input.lines() {
        let matches = coord_re.captures_iter(&line).nth(0);
        match matches {
            Some(cap) => {
                data.push((
                    cap[1].parse::<i32>().unwrap(),
                    cap[2].parse::<i32>().unwrap(),
                    cap[3].parse::<i32>().unwrap(),
                    cap[4].parse::<i32>().unwrap()
                ));
            },
            None => { println!("Bad line: {:?}", line); }
        }
    }
    // println!("{:?}", data);

    let mut no_beacons: HashMap<i32, HashSet<i32>> = HashMap::new();
    let line: i32 = 2000000;
    let x_limit: (i32,i32) = (0, 4000000);
    let y_limit: (i32,i32) = (0, 4000000);
    for sensor in data {
        let dist = (sensor.0 - sensor.2).abs() + (sensor.1 - sensor.3).abs();
        // println!("{:?} is {:?} away", sensor, dist);

        for y in ((sensor.1-dist)..(sensor.1)) {
            // if y != line { continue; }
            if y < y_limit.0 || y > y_limit.1 { continue; }

            no_beacons.entry(y).or_insert(HashSet::new());

            let x1 = sensor.0 - (dist - (sensor.1 - y));
            let x2 = sensor.0 + (dist - (sensor.1 - y));
            for x in (x1..(x2+1)) {
                if x < x_limit.0 || x > x_limit.1 { continue; }

                // if sensor.2 != x || sensor.3 != y {
                //     // This is not a beacon
                //     no_beacon.insert(x);
                // }
                no_beacons.entry(y).and_modify(|s| { s.insert(x); });
            }
        }

        no_beacons.entry(sensor.1).or_insert(HashSet::new());
        for x in ((sensor.0-dist)..(sensor.0+dist+1)) {
            if sensor.1 < y_limit.0 || sensor.1 > y_limit.1 { continue; }
            if x < x_limit.0 || x > x_limit.1 { continue; }

            no_beacons.entry(sensor.1).and_modify(|s| { s.insert(x); });
        }

        for y in ((sensor.1+1)..(sensor.1+dist+1)) {
            // if y != line { continue; }
            if y < y_limit.0 || y > y_limit.1 { continue; }

            no_beacons.entry(y).or_insert(HashSet::new());

            let x1 = sensor.0 - (dist - (y - sensor.1));
            let x2 = sensor.0 + (dist - (y - sensor.1));
            for x in (x1..(x2+1)) {
                if x < x_limit.0 || x > x_limit.1 { continue; }

                // if sensor.2 != x || sensor.3 != y {
                //     // This is not a beacon
                //     no_beacon.insert(x);
                // }

                no_beacons.entry(y).and_modify(|s| { s.insert(x); });
            }
        }
    }

    // println!("Part 1 answer: {:?}", no_beacon.len());

    // KILLED (after maybe 15 min?)

    // Part 2
    for (y, empty) in no_beacons {
        let mut empty_vec: Vec<i32> = empty.into_iter().collect();
        empty_vec.sort();
        let count_diff: i32 = empty_vec[empty_vec.len()-1] - empty_vec[0] + 1;
        if count_diff != (empty_vec.len() as i32) {
            println!("diff mismatch on line {:?}: {:?} != {:?}", y, count_diff, empty_vec.len());
            for x in (0..empty_vec.len()) {
                if x > 0 && empty_vec[x] != (empty_vec[x-1]+1) {
                    println!("Part 2: Empty spot at {:?}, {:?}... frequency is {:?}", x, y, (((x * 4000000) as i32)+y));
                }
            }
        }
    }

}
