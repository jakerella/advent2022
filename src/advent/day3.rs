
pub fn day3(input: String) {
    let priority = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>();

    // Part 1
    let mut total_priority: u16 = 0;
    for pack in input.lines() {
        
        let c1 = &pack[..(pack.len()/2)].chars().collect::<Vec<char>>();
        let c2 = &pack[(pack.len()/2)..].chars().collect::<Vec<char>>();
        
        for c in c1 {
            if c2.contains(c) {
                // println!("Found dupe item: {:?}", c);
                total_priority += (priority.iter().position(|&p| p == *c).unwrap() as u16) + 1;
                break;
            }
        }
    }

    println!("Part 1: total priority: {:?}", total_priority);

    // Part 2
    total_priority = 0;
    let mut group: Vec<String> = Vec::new();

    for pack in input.lines() {
        if group.len() == 3 {
            total_priority += find_badge(&group, &priority);
            group.drain(..);
        }
        group.push(String::from(pack));
    }

    // catch the last group
    if group.len() == 3 {
        total_priority += find_badge(&group, &priority);
    }
    // println!("Last group? {:?}", group);

    println!("Part 2: total priority: {:?}", total_priority);
}

fn find_badge(group: &Vec<String>, priority: &Vec<char>) -> u16 {
    // println!("Finding group badge for: {:?}", group);
    let e1 = group[0].chars().collect::<Vec<char>>();
    let e2 = group[1].chars().collect::<Vec<char>>();
    let e3 = group[2].chars().collect::<Vec<char>>();
    
    let mut group_priority = 0;
    for c in e1 {
        if e2.contains(&c) && e3.contains(&c) {
            // println!("Found badge item: {:?}", c);
            group_priority = (priority.iter().position(|&p| p == c).unwrap() as u16) + 1;
            break;
        }
    }
    return group_priority;
}