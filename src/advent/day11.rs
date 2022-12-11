
use regex::Regex;

pub fn day11(input: String) {
    
    #[derive(Debug, Clone)]
    struct Monkey {
        items: Vec<u128>,
        op: (String, String),
        test: u128,
        true_dest: usize,
        false_dest: usize,
        inspect_count: u32
    };

    let mut monkeys: Vec<Monkey> = Vec::new();

    // Parse out the monkeys...
    for line in input.lines() {
        if String::from(line).is_empty() {
            // println!("Prev Monkey? {:?}", monkeys.last());
            continue;
        }

        if String::from(line).starts_with("Monkey") {
            let mut m = Monkey {
                items: Vec::new(),
                op: (String::from(""), String::from("")),
                test: 1,
                true_dest: 0,
                false_dest: 0,
                inspect_count: 0
            };
            monkeys.push(m);
        }

        let last_i = monkeys.len() - 1;
        let curr_monkey: &mut Monkey = &mut monkeys[last_i];
        
        if String::from(line).starts_with("  Starting items") {
            let items = line.split(": ").collect::<Vec<&str>>()[1].split(", ").collect::<Vec<&str>>();
            for i in items {
                // Only store the modulo since we only care about the divisibility
                // solves part 2 overflows
                let mut item = String::from(i).parse::<u128>().unwrap();
                // item = item % curr_monkey.test;
                curr_monkey.items.push(item);
            }
        }

        if String::from(line).starts_with("  Operation") {
            let op_items = line.split("= ").collect::<Vec<&str>>()[1].split(" ").collect::<Vec<&str>>();
            curr_monkey.op = (String::from(op_items[1]), String::from(op_items[2]));
        }

        if String::from(line).starts_with("  Test") {
            let div_test = line.split(" by ").collect::<Vec<&str>>();
            curr_monkey.test = String::from(div_test[1]).parse::<u128>().unwrap();
        }
        if String::from(line).starts_with("    If true") {
            let test_true = line.split(" monkey ").collect::<Vec<&str>>();
            curr_monkey.true_dest = String::from(test_true[1]).parse::<usize>().unwrap();
        }
        if String::from(line).starts_with("    If false") {
            let test_false = line.split(" monkey ").collect::<Vec<&str>>();
            curr_monkey.false_dest = String::from(test_false[1]).parse::<usize>().unwrap();
        }
    }

    // Run through the rounds of turns...
    // Part 1 = 20 rounds, divisor = 3
    // Part 2 = 10000 rounds, divisor = 1
    let rounds: u16 = 10000;
    let worry_div: u128 = 1;

    // Get the factorial so we can use a least common multiplier for storage later
    let factor: u128 = monkeys.iter().map(|m| m.test).product();

    for r in (0..rounds) {
        for i in (0..monkeys.len()) {
            let monkey_edit: &mut Vec<Monkey> = &mut monkeys;
            for item in monkey_edit[i].items.clone() {
                let mut new_item = calc_worry(item, &monkey_edit[i].op);
                // Part 1
                new_item = (((new_item / worry_div) as f64).floor() as u128);
                
                // Only store the modulo with the LCM since we only care about the divisibility
                // solves part 2 overflows
                new_item = new_item % factor;

                let mut other_monkey = monkey_edit[i].false_dest;
                if new_item % monkey_edit[i].test == 0 {
                    other_monkey = monkey_edit[i].true_dest;
                }
                monkey_edit[other_monkey].items.push(new_item);

                monkey_edit[i].inspect_count += 1;
            }

            monkey_edit[i].items = Vec::new();
        }
    }

    println!("*** Monkeys after {:?} rounds", rounds);
    let mut inspect_counts: Vec<u128> = Vec::new();
    for m in &monkeys {
        println!("{:?}", m);
        inspect_counts.push(m.inspect_count as u128);
    }
    inspect_counts.sort();
    inspect_counts = inspect_counts.into_iter().rev().collect();
    println!("inspect counts sorted: {:?}", inspect_counts);

    let monkey_biz:u128 = inspect_counts[0] * inspect_counts[1];
    println!("Part 1 answer: {:?}", monkey_biz);

}

fn calc_worry(worry: u128, op: &(String, String)) -> u128 {
    let mut old_value = worry.clone();
    if op.1 != "old" {
        old_value = String::from(op.1.clone()).parse::<u128>().unwrap();
    }

    if op.0 == "*" {
        return old_value * worry;
    } else if op.0 == "+" {
        return old_value + worry;
    } else {
        println!("Bad op: {:?}", op.0);
        return old_value;
    }
}
