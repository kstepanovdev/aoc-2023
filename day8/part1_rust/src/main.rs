use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let binding = read_to_string("../input.txt").unwrap();
    let file = binding.split("\n\n").collect::<Vec<&str>>();
    let mut map = HashMap::new();
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
    });

    let mut steps = 0;
    let mut curr = "AAA";

    while curr != "ZZZ" {
        curr = if directions[steps % directions.len()] == 'L' {
            map.get(&curr).unwrap().0
        } else {
            map.get(&curr).unwrap().1
        };
        steps += 1;
    }

    println!("{}", steps);
}
