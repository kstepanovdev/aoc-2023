use std::fs::read_to_string;

fn main() {
    let binding = read_to_string("../input.txt").unwrap();
    let mut file = binding.split('\n');
    let time = file
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect::<String>()
        .parse::<f64>()
        .unwrap();

    let record = file
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect::<String>()
        .parse::<f64>()
        .unwrap();

    let (x1, x2) = solve_quadratic(time, record);

    println!("{:?}", (x1.floor() as u64) - (x2.ceil() as u64) + 1);
}

fn solve_quadratic(time: f64, record: f64) -> (f64, f64) {
    // general equation for gathering the res is: x*(time-x)=distance
    // transfrom it to x^2 - x*time + dist = 0

    // because coef. near x is 1 and so on
    let a = 1.0;
    let b = -1.0 * time;
    let c = record;

    let sqrt_d = (b * b - 4.0 * a * c).sqrt();

    let x1 = (-1.0 * b + sqrt_d) / (2.0 * a);
    let x2 = (-1.0 * b - sqrt_d) / (2.0 * a);
    (x1, x2)
}
