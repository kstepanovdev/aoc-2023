use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let mut unplayed_cards = HashMap::new();

    if let Ok(lines) = read_lines("../input.txt") {
        for (mut n_card, line) in lines.flatten().enumerate() {
            n_card += 1;
            if let Some(prev_val) = unplayed_cards.get(&n_card) {
                unplayed_cards.insert(n_card, *prev_val + 1);
            } else {
                unplayed_cards.insert(n_card, 1);
            }

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
            let winning_len = winning_cards
                .intersection(&owned_cards)
                .collect::<HashSet<_>>()
                .len();

            let times = *unplayed_cards.get(&n_card).unwrap();

            for k in (n_card + 1)..n_card + 1 + winning_len {
                if let Some(prev_val) = unplayed_cards.get(&k) {
                    unplayed_cards.insert(k, *prev_val + times);
                } else {
                    unplayed_cards.insert(k, times);
                }
            }
        }
    }

    println!("{:?}", unplayed_cards.values().sum::<i32>());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
