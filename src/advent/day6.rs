
pub fn day6(input: String) {
    
    let buffer = input.chars();
    // println!("buffer: {:?}", buffer);
    
    // Part 1
    let mut last_four: Vec<char> = Vec::new();
    let mut last_index: u16 = 0;
    for c in buffer {
        // println!("last four: {:?}, looking at {:?}", last_four, c);
        let dupe_found = last_four.iter().position(|&v| v == c);
        match dupe_found {
            Some(index) => {
                // println!("found dupe: {:?} at index {:?}", &c, &index);
                for i in (0..(index+1)) {
                    // println!("removing first entry");
                    last_four.remove(0);
                }
            },
            None => {
                if last_four.len() == 3 {
                    last_four.push(c);
                    last_index += 1;
                    break;
                }
            }
        }

        last_four.push(c);
        last_index += 1;
    }

    println!("Part 1 answer: {:?} (signal: {:?})", last_index, last_four);

    let mut last_fourteen: Vec<char> = Vec::new();
    last_index = 0;
    let new_buffer = input.chars();
    for c in new_buffer {
        // println!("last four: {:?}, looking at {:?}", last_four, c);
        let dupe_found = last_fourteen.iter().position(|&v| v == c);
        match dupe_found {
            Some(index) => {
                // println!("found dupe: {:?} at index {:?}", &c, &index);
                for i in (0..(index+1)) {
                    // println!("removing first entry");
                    last_fourteen.remove(0);
                }
            },
            None => {
                if last_fourteen.len() == 13 {
                    last_fourteen.push(c);
                    last_index += 1;
                    break;
                }
            }
        }

        last_fourteen.push(c);
        last_index += 1;
    }

    println!("Part 2 answer: {:?} (signal: {:?})", last_index, last_fourteen);
}
