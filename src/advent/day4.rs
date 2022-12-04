
pub fn day4(input: String) {

    let mut full_overlaps = 0;
    let mut any_overlaps = 0;
    
    for pair in input.lines() {
        let elves = pair.split(",").collect::<Vec<&str>>();
        let elf1_sectors = elves[0].split("-").collect::<Vec<&str>>();
        let elf2_sectors = elves[1].split("-").collect::<Vec<&str>>();
        let e1_start: u8 = elf1_sectors[0].parse().unwrap();
        let e1_end: u8 = elf1_sectors[1].parse().unwrap();
        let e2_start: u8 = elf2_sectors[0].parse().unwrap();
        let e2_end: u8 = elf2_sectors[1].parse().unwrap();

        if e1_start >= e2_start && e1_end <= e2_end {
            full_overlaps += 1;
        } else if e2_start >= e1_start && e2_end <= e1_end {
            full_overlaps += 1;
        }

        if e1_start >= e2_start && e1_start <= e2_end {
            any_overlaps += 1;
        } else if e2_start >= e1_start && e2_start <= e1_end {
            any_overlaps += 1;
        }
    }

    println!("Part 1: full overlaps: {:?}", full_overlaps);
    println!("Part 2: any overlaps: {:?}", any_overlaps);
}
