
use std::num;
use std::collections::HashSet;
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

    let mut no_beacon: HashSet<i32> = HashSet::new();
    let line: i32 = 2000000;
    for sensor in data {
        let dist = (sensor.0 - sensor.2).abs() + (sensor.1 - sensor.3).abs();
        // println!("{:?} is {:?} away", sensor, dist);

        if sensor.1 < line { // only need to look down
            for y in ((sensor.1+1)..(sensor.1+dist+1)) {
                if y != line { continue; }
                let x1 = sensor.0 - (dist - (y - sensor.1));
                let x2 = sensor.0 + (dist - (y - sensor.1));
                for x in (x1..(x2+1)) {
                    if sensor.2 != x || sensor.3 != y {
                        // This is not a beacon
                        no_beacon.insert(x);
                    }
                }
            }

        } else  { // only need to look up
            for y in ((sensor.1-dist)..(sensor.1)) {
                if y != line { continue; }
                let x1 = sensor.0 - (dist - (sensor.1 - y));
                let x2 = sensor.0 + (dist - (sensor.1 - y));
                for x in (x1..(x2+1)) {
                    if sensor.2 != x || sensor.3 != y {
                        // This is not a beacon
                        no_beacon.insert(x);
                    }
                }
            }
        }
    }
    // println!("x locations on line {:?} are {:?}", line, no_beacon);

    println!("Part 1 answer: {:?}", no_beacon.len());

}
