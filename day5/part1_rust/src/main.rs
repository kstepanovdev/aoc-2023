use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Debug)]
struct Mapping {
    min_curr_lock: u64,
    min_next_lock: u64,
    mapping_size: u64,
}

impl Mapping {
    fn contains_seed(&self, seed_loc: u64) -> bool {
        let max_current_lock = self.min_curr_lock + self.mapping_size;
        seed_loc >= self.min_curr_lock && seed_loc < max_current_lock
    }

    fn get_next_loc(&self, seed_loc: u64) -> u64 {
        let abs = if seed_loc > self.min_curr_lock {
            seed_loc - self.min_curr_lock
        } else {
            self.min_curr_lock - seed_loc
        };

        self.min_next_lock + abs
    }
}

fn main() {
    let mut seed_loc = HashMap::new();
    let mut seeds = vec![];
    let mut curr_mapping = vec![];
    let mut seeds_collected = false;
    let mut junk = true;

    if let Ok(lines) = read_lines("../input.txt") {
        for line in lines.flatten() {
            if line.starts_with("seeds: ") {
                line.trim()
                    .split(':')
                    .nth(1)
                    .unwrap()
                    .split_whitespace()
                    .map(|num| num.parse::<u64>().unwrap())
                    .for_each(|num| {
                        seeds.push(num);
                        seed_loc.insert(num, num);
                    });
            } else if line.is_empty() {
                if !seeds_collected {
                    seeds_collected = true;
                    continue;
                }

                populate_next_seeds(&seeds, &mut seed_loc, &curr_mapping);
                curr_mapping = vec![];
                junk = true;
            } else {
                if junk {
                    junk = false;
                    continue;
                }

                let mut nums = line.split_whitespace().map(|el| el.parse::<u64>().unwrap());
                let mapping = Mapping { min_next_lock: nums.next().unwrap(), min_curr_lock: nums.next().unwrap(), mapping_size: nums.next().unwrap() };
                curr_mapping.push(mapping);
            }
        }
    }
    populate_next_seeds(&seeds, &mut seed_loc, &curr_mapping);
    println!("{:?}", seed_loc.values().min());
}

fn populate_next_seeds(
    seeds: &Vec<u64>,
    seed_loc: &mut HashMap<u64, u64>,
    curr_mapping: &Vec<Mapping>,
) {
    for seed in seeds {
        let last_seed_loc = *seed_loc.get(seed).unwrap();

        for mapping in curr_mapping {
            if mapping.contains_seed(last_seed_loc) {
                seed_loc.insert(*seed, mapping.get_next_loc(last_seed_loc));
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
