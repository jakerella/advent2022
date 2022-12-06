
pub fn day6(input: String) {
    println!("Part 1 answer: {:?}", find_start(input.clone(), 4));
    println!("Part 2 answer: {:?}", find_start(input, 14));
}


fn find_start(input: String, limit: u16) -> u16 {
    let mut last_set: Vec<char> = Vec::new();
    let mut last_index = 0;
    let buffer = input.chars();
    for c in buffer {
        // println!("last set: {:?}, looking at {:?}", last_set, c);
        let dupe_found = last_set.iter().position(|&v| v == c);
        match dupe_found {
            Some(index) => {
                // println!("found dupe: {:?} at index {:?}", &c, &index);
                for i in (0..(index+1)) {
                    // println!("removing first entry");
                    last_set.remove(0);
                }
            },
            None => {
                if last_set.len() == usize::from(limit - 1) {
                    last_set.push(c);
                    last_index += 1;
                    break;
                }
            }
        }

        last_set.push(c);
        last_index += 1;
    }

    return last_index;
}