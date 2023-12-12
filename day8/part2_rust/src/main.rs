use std::{collections::HashMap, fs::read_to_string};

use num::Integer;

fn main() {
    let binding = read_to_string("../input.txt").unwrap();
    let file = binding.split("\n\n").collect::<Vec<&str>>();
    let mut map = HashMap::new();
    let mut curr_positions = vec![];
    let mut steps = 0;
    let directions: Vec<char> = file[0].chars().collect();

    file[1].split('\n').for_each(|chunk| {
        let (from, to) = chunk.split_once(" = ").unwrap();
        let to = to
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split_once(", ")
            .unwrap();

        map.insert(from, to);

        if from.ends_with('A') {
            curr_positions.push(from)
        }
    });

    let mut steps_counter = 0;
    for curr in curr_positions {
        let mut curr = curr;
        while !curr.ends_with('Z') {
            curr = if directions[steps_counter % directions.len()] == 'L' {
                map.get(&curr).unwrap().0
            } else {
                map.get(&curr).unwrap().1
            };
            steps_counter += 1;
        }

        steps = if steps == 0 {
            steps_counter
        } else {
            steps.lcm(&steps_counter)
        };
        steps_counter = 0;
    }

    println!("{}", steps);
}
