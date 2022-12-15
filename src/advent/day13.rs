
pub fn day13(input: String) {
    
    let pairs = input.split("\n\n").collect::<Vec<&str>>();

    let mut ordered: Vec<usize> = Vec::new();
    for pair in pairs {
        let pair_s = String::from(pair);
        let pair_s = pair_s.split("\n").collect::<Vec<&str>>();
        // let top = String::from(pair_s[0]);
        let bottom = String::from(pair_s[0]);

        // can give a type annotation because of the multiple nesting...
        // Hmm....
        // let top = serde_json::from_str(pair_s[0]).unwrap();
        
        // let top = top.chars();
        // let bottom = bottom.chars();
        // println!("pair: {:?} .. {:?}", top, bottom);
    }

    println!("Part 1 answer: {:?}", ordered.iter().sum::<usize>());
}
