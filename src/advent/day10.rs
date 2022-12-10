
use itertools::Itertools;

pub fn day10(input: String) {
    
    let mut reg_x: i16 = 1;
    let mut cycle: i16 = 1;
    let mut strengths: Vec<(i16, i16)> = Vec::new();
    let mut crt: Vec<&str> = Vec::from([" "; 240]);

    for line in input.lines() {
        let cmd = line.split(" ").collect::<Vec<&str>>();
        let instr = cmd[0];
        let mut v: i16 = 0;
        
        match cmd.get(1) {
            Some(n) => { v = n.parse().unwrap() },
            None => {}
        }

        if instr == "noop" {
            // Part 1
            if cycle == 20 || (cycle-20) % 40 == 0 {
                strengths.push((cycle, reg_x));
            }
            // Part 2
            draw_px(cycle, reg_x, &mut crt);

            cycle += 1;

        } else if instr == "addx" {
            // Part 1
            if (cycle == 20 || (cycle-20) % 40 == 0) {
                // println!("at cycle {:?}, curr instr: {:?} {:?}, register value: {:?}", cycle, instr, v, reg_x);
                strengths.push((cycle, reg_x));
            } else if (cycle+1) == 20 || (cycle-20+1) % 40 == 0 {
                strengths.push((cycle+1, reg_x));
            }

            // Part 2
            draw_px(cycle, reg_x, &mut crt);
            cycle +=1;
            draw_px(cycle, reg_x, &mut crt);

            reg_x += v;
            cycle +=1;

        } else {
            println!("bad instr {:?} with v={:?}", instr, v);
        }
        
        // println!("executing {:?} with v={:?}", instr, v);
    }

    let mut total_strength:i16 = 0;
    for s in strengths {
        // println!("cycle {:?} has register value {:?}", &s.0, &s.1);
        total_strength += s.0 * s.1;
    }

    println!("Part 1 answer: {:?}", total_strength);


    // Part 2
    println!("Part 2 answer:");
    let all_chars = crt.join("");
    for chunk in &all_chars.chars().chunks(40) {
        println!("{:?}", String::from_iter(chunk));
    }

}


fn draw_px(cycle: i16, reg_x: i16, crt: &mut Vec<&str>) {
    let line_pos: i16 = (cycle % 40) - 1;
    let px = usize::from((cycle as u16) - 1);
    // if reg_x >= ((line_pos - 1) as i16) && reg_x <= ((line_pos + 1) as i16) {
    if line_pos >= ((reg_x - 1) as i16) && line_pos <= ((reg_x + 1) as i16) {
        // println!("drawing # at {:?} which is line_pos: {:?}, {:?}", px, ((cycle / 40) as f32).floor(), line_pos);
        crt[px] = "#";
    } else {
        crt[px] = ".";
    }
    // if cycle < 41 {
    //     println!("draw px on cycle {:?}? line_pos {:?} btw {:?} && {:?}", cycle, line_pos, reg_x-1, reg_x+1);
    //     println!("{:?}", crt.join(""));
    // }
}
