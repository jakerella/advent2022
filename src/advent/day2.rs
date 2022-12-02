use std::collections::HashMap;


pub fn day2(input: String) {

    let points = HashMap::from([("X", 1), ("Y", 2), ("Z", 3), ("A", 1), ("B", 2), ("C", 3)]);

    // Part 1 pieces
    let opp = HashMap::from([("A", "Y"), ("B", "Z"), ("C", "X")]);
    let same = HashMap::from([("A", "X"), ("B", "Y"), ("C", "Z")]);

    // Part 2 pieces
    let win = HashMap::from([("A", "B"), ("B", "C"), ("C", "A")]);
    let lose = HashMap::from([("A", "C"), ("B", "A"), ("C", "B")]);

    let mut total_p1 = 0;
    let mut total_p2 = 0;

    for round in input.lines() {
        let moves = round.split(" ");
        let moves = moves.collect::<Vec<&str>>();

        // Part 1
        match points.get(moves[1]) {
            Some(p) => total_p1 += p,
            None => { println!("bad move: {:?}", moves); }
        }
        match opp.get(moves[0]) {
            Some(&w) => {
                if w == moves[1] { total_p1 += 6; }
            },
            None => { println!("bad move: {:?}", moves); }
        }
        match same.get(moves[0]) {
            Some(&t) => {
                if t == moves[1] { total_p1 += 3; }
            },
            None => { println!("bad move: {:?}", moves); }
        }

        // Part 2
        if moves[1] == "X" {
            match lose.get(moves[0]) {
                Some(&play) => {
                    match points.get(play) {
                        Some(p) => total_p2 += p,
                        None => { println!("bad points for: {:?}", play); }
                    }
                },
                None => { println!("bad lose move: {:?}", moves); }
            }
        } else if moves[1] == "Y" {
            total_p2 += 3;
            match points.get(moves[0]) {
                Some(p) => total_p2 += p,
                None => { println!("bad tie move: {:?}", moves); }
            }
        } else if moves[1] == "Z" {
            total_p2 += 6;
            match win.get(moves[0]) {
                Some(&play) => {
                    match points.get(play) {
                        Some(p) => total_p2 += p,
                        None => { println!("bad points for: {:?}", play); }
                    }
                },
                None => { println!("bad win move: {:?}", moves); }
            }
        }

    }

    println!("Part 1: total score: {:?}", total_p1);
    println!("Part 2: total score: {:?}", total_p2);
}
