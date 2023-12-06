use std::fs::read_to_string;

fn main() {
    let binding = read_to_string("../input.txt").unwrap();
    let mut file = binding.split('\n');
    let seconds = file
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(|el| el.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let records = file
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(|el| el.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut result = 1;
    for race_i in 0..seconds.len() {
        let mut subres = 0;
        let mut left = 1;
        let mut right = seconds[race_i] - 1;

        while right >= left {
            if (left * right) > records[race_i] {
                subres += 1;
            }

            left += 1;
            right -= 1;
        }

        if seconds[race_i] % 2 == 0 {
            subres = subres * 2 - 1;
        } else {
            subres *= 2;
        }
        println!("{subres}");
        result *= subres;
    }
    println!("{:?}", result);
}
