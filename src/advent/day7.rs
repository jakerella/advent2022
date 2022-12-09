
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone)]
struct Dir {
    name: String,
    size: u32,
    children: Vec<String>
}

pub fn day7(input: String) {

    let fs_re = Regex::new(r"^(\$|dir|\d+) (cd|ls|[a-z\.]+)(?: ([a-z\./]+))?$").unwrap();
    let size_re = Regex::new(r"^\d+$").unwrap();

    let mut fs: HashMap<String, Dir> = HashMap::new();
    let mut path: Vec<String> = Vec::new();

    for line in input.lines() {
        if fs_re.is_match(&line.to_string()) {
            let fs_re_match = fs_re.captures_iter(&line[..]).nth(0);
            match fs_re_match {
                Some(fs_match) => {
                    let part1 = String::from(&fs_match[1]);
                    let part2 = String::from(&fs_match[2]);
                    if part1 == "$" && part2 == "cd" {
                        let dir = String::from(&fs_match[3]);
                        if dir == ".." {
                            path.pop();
                            // println!("going up, now at {:?}", path);
                        } else {
                            path.push(String::from(&dir));
                            // println!("going into {:?}, now at {:?}", dir, path);
                        }
                        fs.entry(path.join("-")).or_insert(Dir {
                            name: path.join("-"),
                            size: 0,
                            children: Vec::new()
                        }).clone();
                    
                    } else if part1 == "dir" {
                        // println!("adding dir {:?} to {:?}", part2, path.join("-"));
                        fs.entry(path.join("-")).and_modify(|d| d.children.push(part2));

                    // } else if part1 == "$" && part2 == "ls" {
                    //     println!("listing dir: {:?}", path);
                    } else if size_re.is_match(&part1) {
                        let size = &part1.parse::<u32>().unwrap();
                        // println!("adding filesize {:?} to {:?}", size, path.join("-"));
                        fs.entry(path.join("-")).and_modify(|d| d.size += size);
                    }
                },
                None => { println!("bad line? {:?}", line); }
            }
        } else {
            println!("no line match? {:?}", line);
        }
    }

    // println!("my fs: {:?}", fs);

    let mut dir_sizes: HashMap<String, u32> = HashMap::new();
    get_total_dir_size(&fs, String::from("/"), 0, &mut dir_sizes);

    // println!("dir sizes: {:?}", dir_sizes);
    let mut small_dir_total = 0;
    for (d,s) in dir_sizes {
        if s < 100000 {
            small_dir_total += s;
        }
    }

    println!("Part 1 answer: {:?}", small_dir_total);
}


fn get_total_dir_size(fs: &HashMap<String, Dir>, dir: String, sum: u32, dir_sizes: &mut HashMap<String, u32>) -> u32 {
    // println!("checking size of {:?}", dir);
    
    let mut total = sum;
    let fs_dir = fs.get(&dir);
    match fs_dir {
        Some(d) => {
            total += d.size;
            for c in &d.children {

                total += get_total_dir_size(fs, [dir.to_string(), c.to_string()].join("-"), sum, dir_sizes);
            }
        },
        None => {}
    }

    // println!("total size of {:?} is {:?}", dir, total);
    dir_sizes.insert(dir, total);
    return total;
}
