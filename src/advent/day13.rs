
pub fn day13(input: String) {
    
    let no_tens = input.replace("10", "a");
    let pairs = no_tens.split("\n\n").collect::<Vec<&str>>();
    let mut ordered: Vec<usize> = Vec::new();

    for (i, block) in pairs.iter().enumerate() {
        let pair = String::from(*block);
        let pair = pair.split("\n").collect::<Vec<&str>>();
        // println!("pair: {:?}", pair);

        if is_ordered(pair[0].as_bytes(), pair[1].as_bytes()) {
            ordered.push(i+1);
        }
    }

    println!("ordered indices: {:?}", ordered);
    println!("Part 1 answer: {:?}", ordered.iter().sum::<usize>());

}

fn is_ordered(top: &[u8], bottom: &[u8]) -> bool {
    match (top[0], bottom[0]) {
        // if the integers are equal (or both starting brackets or commas) move on
        (a, b) if a == b => is_ordered(&top[1..], &bottom[1..]),
        (_, b']') => false, // bottom ended first
        (b']', _) => true, // top ended first
        (b'[', _) => {
            // bottom was not an array
            let new_bottom = [&[bottom[0], b']'], &bottom[1..]].concat();
            return is_ordered(&top[1..], &new_bottom);
        },
        (_, b'[') => {
            // top was not an array
            let new_top = [&[top[0], b']'], &top[1..]].concat();
            return is_ordered(&new_top, &bottom[1..]);
        },
        (_, _) => top[0].cmp(&bottom[0]).is_lt()
    }
}
