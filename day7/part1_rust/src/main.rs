use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use once_cell::sync::Lazy;

#[derive(Debug, Eq, Ord)]
struct Hand {
    cards: Vec<char>,
    bid: i32,
    combinaton: i32,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.combinaton != other.combinaton {
            return false;
        }

        for (i, c) in self.cards.iter().enumerate() {
            if *c != other.cards[i] {
                return false;
            }
        }

        true
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.combinaton.partial_cmp(&other.combinaton) {
            Some(Ordering::Equal) => Some(compare_hands(&self.cards, &other.cards)),
            Some(ordering) => Some(ordering),
            None => unreachable!(),
        }
    }
}

fn compare_hands(a: &[char], b: &[char]) -> Ordering {
    let (a, b) = a
        .iter()
        .zip(b)
        .find(|(a, b)| a != b)
        .expect("hands are identical");

    CARDS_SCORE.get(a).unwrap().cmp(CARDS_SCORE.get(b).unwrap())
}

static CARDS_SCORE: Lazy<HashMap<char, i32>> = Lazy::new(|| {
    HashMap::from([
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('J', 10),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('5', 4),
        ('4', 3),
        ('3', 2),
        ('2', 1),
    ])
});

fn main() {
    let mut hands = vec![];

    if let Ok(lines) = read_lines("../input.txt") {
        for line in lines.flatten() {
            let line: Vec<&str> = line.split_whitespace().collect();
            hands.push(Hand {
                cards: line[0].chars().collect(),
                bid: line[1].parse().unwrap(),
                combinaton: 0,
            });
        }
    }

    for hand in &mut hands {
        let mut cards_nominals = HashMap::new();
        for card in &hand.cards {
            if let Some(value) = cards_nominals.get_mut(&card) {
                *value += 1;
            } else {
                cards_nominals.insert(card, 1);
            }
        }

        /*
        1 = five
        2 = full house || four
        3 = two pair || three
        4 = pair
        5 = kicker
        */
        let combination_length = cards_nominals.keys().len();
        hand.combinaton = match combination_length {
            1 => 7,
            2 => {
                if cards_nominals.values().any(|val| *val == 4) {
                    6
                } else {
                    5
                }
            }
            3 => {
                if cards_nominals.values().any(|val| *val == 3) {
                    4
                } else {
                    3
                }
            }
            4 => 2,
            _ => 1,
        }
    }
    hands.sort();
    let mut res = 0;
    for (i, hand) in hands.iter().enumerate() {
        res += hand.bid * (i32::try_from(i).unwrap() + 1);
    }

    // println!("{:?}", hands);
    println!("{:?}", res);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
