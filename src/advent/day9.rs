
pub fn day9(input: String) {
    
    let mut head: (i16, i16) = (0, 0);
    let mut tail: (i16, i16) = (0, 0);

    let mut tracker: Vec<String> = Vec::new();
    tracker.push(String::from("0-0"));

    for line in input.lines() {
        let instr = line.split(" ").collect::<Vec<&str>>();
        let dir = instr[0];
        let amt: i16 = instr[1].parse().unwrap();

        // println!("moving head {:?} to the {:?}", amt, dir);
        for a in (0..amt) {
            if dir == "R" {
                head.1 += 1;
            } else if dir == "L" {
                head.1 -= 1;
            } else if dir == "U" {
                head.0 -= 1;
            } else if dir == "D" {
                head.0 += 1;
            }
            move_tail(&head, &mut tail);
            let tail_pos = [tail.0.to_string(), tail.1.to_string()].join("-");
            if !tracker.contains(&tail_pos) {
                tracker.push(tail_pos);
            }
        }

        // println!("new pos: head {:?} and tail {:?}", head, tail);
    }

    // println!("tail tracker: {:?}", tracker);

    println!("Part 1 tail visits: {:?}", tracker.len());

    head = (0,0);
    let mut tails: Vec<(i16, i16)> = Vec::from([(0,0); 9]);
    tracker = Vec::new();

    for line in input.lines() {
        let instr = line.split(" ").collect::<Vec<&str>>();
        let dir = instr[0];
        let amt: i16 = instr[1].parse().unwrap();

        for a in (0..amt) {
            if dir == "R" {
                head.1 += 1;
            } else if dir == "L" {
                head.1 -= 1;
            } else if dir == "U" {
                head.0 -= 1;
            } else if dir == "D" {
                head.0 += 1;
            }
            move_all_tails(&head, &mut tails);
            let tail_pos = [tails[8].0.to_string(), tails[8].1.to_string()].join("-");
            if !tracker.contains(&tail_pos) {
                tracker.push(tail_pos);
            }
        }

        // println!("new pos: head {:?} and tail {:?}", head, tails[8]);
    }

    println!("Part 2 tail visits: {:?}", tracker.len());
}

fn move_all_tails(head: &(i16, i16), tails: &mut Vec<(i16, i16)>) {
    let mut curr_head: (i16, i16) = *head;
    for mut tail in tails {
        move_tail(&curr_head, &mut tail);
        curr_head = tail.clone();
    }
}

fn move_tail(head: &(i16, i16), tail: &mut (i16, i16)) {
    if head.0 == tail.0 {  // same row
        if head.1 > (tail.1 + 1) { tail.1 += 1; }
        if head.1 < (tail.1 - 1) { tail.1 -= 1; }
    } else if head.1 == tail.1 {  // same col
        if head.0 > (tail.0 + 1) { tail.0 += 1; }
        if head.0 < (tail.0 - 1) { tail.0 -= 1; }
    } else if head.0 < tail.0 {  // head above tail
        if head.1 > tail.1 {  // head above and right
            if head.1 > (tail.1 + 1) || head.0 < (tail.0 - 1) {
                tail.0 -= 1; tail.1 += 1;
            }
        }
        if head.1 < tail.1 {  // head above and left
            if head.1 < (tail.1 - 1) || head.0 < (tail.0 - 1) {
                tail.0 -= 1; tail.1 -= 1;
            }
        }
        
    } else if head.0 > tail.0 {  // head below tail
        if head.1 > tail.1 {  // head below and right
            if head.1 > (tail.1 + 1) || head.0 > (tail.0 + 1) {
                tail.0 += 1; tail.1 += 1;
            }
        }
        if head.1 < tail.1 {  // head nelow and left
            if head.1 < (tail.1 - 1) || head.0 > (tail.0 + 1) {
                tail.0 += 1; tail.1 -= 1;
            }
        }
        
    }
}
