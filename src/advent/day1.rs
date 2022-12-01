
pub fn day1(input: String) {
    
    let mut max_cal: u32 = 0;
    let mut top_three: [u32; 3] = [0; 3];

    let mut curr_cal: u32 = 0;
    for line in input.lines() {
        if String::from(line).is_empty() {
            
            // Part 1
            if curr_cal > max_cal {
                max_cal = curr_cal;
            }

            // Part 2
            if curr_cal > top_three[2] {
                top_three[2] = curr_cal;
                top_three.sort();
                top_three.reverse();
            }

            curr_cal = 0;
            continue;
        }
        curr_cal += String::from(line).parse::<u32>().unwrap();
    }

    println!("Part 1: max single-elf calories: {:?}", max_cal);

    println!("Part 2: top-three elf calories: {:?} ({:?})", top_three.iter().sum::<u32>(), top_three);
}
