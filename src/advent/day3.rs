
pub fn day3(input: String) {
    let priority = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>();

    let mut total_priority = 0;
    for pack in input.lines() {
        
        let c1 = &pack[..(pack.len()/2)].chars().collect::<Vec<char>>();
        let c2 = &pack[(pack.len()/2)..].chars().collect::<Vec<char>>();
        
        for c in c1 {
            if c2.contains(c) {
                // println!("Found dupe item: {:?}", c);
                total_priority += priority.iter().position(|&p| p == *c).unwrap() + 1;
                break;
            }
        }
    }

    println!("Part 1: total priority: {:?}", total_priority);
}
