use std::{cmp::Ordering, ops::Index};


pub fn day13(input: String) {
    
    let no_tens = input.replace("10", "a");
    let pairs = no_tens.split("\n\n").collect::<Vec<&str>>();
    let mut ordered_indices: Vec<usize> = Vec::new();

    for (i, block) in pairs.iter().enumerate() {
        let pair = String::from(*block);
        let pair = pair.split("\n").collect::<Vec<&str>>();
        // println!("pair: {:?}", pair);

        if is_ordered(pair[0].as_bytes(), pair[1].as_bytes()) {
            ordered_indices.push(i+1);
        }
    }
    // println!("ordered indices: {:?}", ordered_indices);
    println!("Part 1 answer: {:?}", ordered_indices.iter().sum::<usize>());


    // Part 2

    let mut packets = no_tens
        .split("\n")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();
    // add the divider packets
    packets.push("[[2]]");
    packets.push("[[6]]");


    packets.sort_by(|a, b| {
        if is_ordered(a.as_bytes(), b.as_bytes()) {
            return Ordering::Less
        } else {
            return Ordering::Greater
        }
    });

    // println!("sorted packets: {:?}", packets);

    let div_2_i = packets.iter().position(|&v| v == "[[2]]").unwrap() + 1;
    let div_6_i = packets.iter().position(|&v| v == "[[6]]").unwrap() + 1;
    println!("Part 2 answer: {:?}", div_2_i * div_6_i);

}

fn is_ordered(top: &[u8], bottom: &[u8]) -> bool {
    match (top[0], bottom[0]) {
        // if the integers are equal (or both starting brackets or commas) move on
        (a, b) if a == b => is_ordered(&top[1..], &bottom[1..]),
        (_, b']') => false, // bottom ended first
        (b']', _) => true, // top ended first
        (b'[', _) => {
            // bottom was not an array
            let new_bottom = [&[bottom[0], b']'], &bottom[1..]].concat();
            return is_ordered(&top[1..], &new_bottom);
        },
        (_, b'[') => {
            // top was not an array
            let new_top = [&[top[0], b']'], &top[1..]].concat();
            return is_ordered(&new_top, &bottom[1..]);
        },
        (_, _) => top[0].cmp(&bottom[0]).is_lt()
    }
}
