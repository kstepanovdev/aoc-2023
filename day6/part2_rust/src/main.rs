use std::fs::read_to_string;

fn main() {
    let binding = read_to_string("../input.txt").unwrap();
    let mut file = binding.split('\n');
    let seconds = file
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let record = file
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let mut result = 0;
    let mut left = 1;
    let mut right = seconds - 1;

    while right >= left {
        if (left * right) > record {
            result += 1;
        }

        left += 1;
        right -= 1;
    }

    if seconds % 2 == 0 {
        result = result * 2 - 1;
    } else {
        result *= 2;
    }

    println!("{:?}", result);
}
