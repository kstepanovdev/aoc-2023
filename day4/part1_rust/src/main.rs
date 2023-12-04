use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let mut result = 0;

    if let Ok(lines) = read_lines("../input.txt") {
        for line in lines.flatten() {
            let game = line.split(':').nth(1).map_or(Vec::new(), |split_str| {
                split_str
                    .split('|')
                    .map(|s| {
                        s.split_whitespace()
                            .map(|el| el.parse::<u32>().unwrap())
                            .collect::<Vec<u32>>()
                    })
                    .collect::<Vec<Vec<u32>>>()
            });
            let winning_cards = HashSet::<u32>::from_iter(game[0].clone());
            let owned_cards = HashSet::<u32>::from_iter(game[1].clone());
            let winning_len: u32 = winning_cards
                .intersection(&owned_cards)
                .collect::<HashSet<_>>()
                .len()
                .try_into()
                .unwrap();
            if winning_len > 0 {
                result += 2_u32.pow(winning_len - 1);
            }
        }
    }

    println!("{}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
