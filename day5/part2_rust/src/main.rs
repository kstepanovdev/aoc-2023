use parking_lot::Mutex;
use rayon::prelude::*;
use std::{fs, sync::Arc};

#[derive(Debug)]
struct Mapping {
    current: u64,
    next: u64,
    range: u64,
}

impl Mapping {
    fn contains_seed(&self, seed_loc: u64) -> bool {
        let max_current_lock = self.current + self.range;
        seed_loc >= self.current && seed_loc < max_current_lock
    }

    fn next_loc(&self, seed_loc: u64) -> u64 {
        let abs = if seed_loc > self.current {
            seed_loc - self.current
        } else {
            self.current - seed_loc
        };

        self.next + abs
    }
}

#[derive(Debug)]
struct SeedRange {
    start: u64,
    end: u64,
}

fn main() {
    let mut mapping_steps = vec![];
    let min_placing = Arc::new(Mutex::new(u64::MAX));

    let file = fs::read_to_string("../input.txt").unwrap();
    let (seeds_str, maps_str) = file.split_once("\n\n").unwrap();
    let seed_ranges = seeds_str
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|num_str| num_str.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|chunk| SeedRange {
            start: chunk[0],
            end: chunk[0] + chunk[1],
        })
        .collect::<Vec<SeedRange>>();

    for block in maps_str.split("\n\n") {
        // ignore header of map
        let (_, maps) = block.split_once('\n').unwrap();
        let mut subrule = vec![];
        for map in maps
            .split_whitespace()
            .map(|num_str| num_str.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
            .chunks(3)
        {
            let mapping = Mapping {
                current: map[1],
                next: map[0],
                range: map[2],
            };
            subrule.push(mapping);
        }
        mapping_steps.push(subrule);
    }

    seed_ranges.into_par_iter().for_each(|seed_range| {
        (seed_range.start..seed_range.end)
            .into_par_iter()
            .for_each(|seed| {
                let mut last_loc = seed;

                for mapping_step in &mapping_steps {
                    for mapping in mapping_step {
                        if mapping.contains_seed(last_loc) {
                            last_loc = mapping.next_loc(last_loc);
                            break;
                        }
                    }
                }
                let mut data = min_placing.lock();
                *data = data.min(last_loc);
            });
    });

    println!("{:?}", min_placing.lock());
}
