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
    j_number: usize,
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
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
        ('J', 1),
    ])
});

fn main() {
    let mut hands = vec![];

    if let Ok(lines) = read_lines("../input.txt") {
        for line in lines.flatten() {
            let line: Vec<&str> = line.split_whitespace().collect();
            let j_number = line[0].chars().filter(|c| c == &'J').count();
            hands.push(Hand {
                cards: line[0].chars().collect(),
                bid: line[1].parse().unwrap(),
                combinaton: 0,
                j_number,
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

        let js: i32 = hand.j_number.try_into().unwrap();
        let combination_len = cards_nominals.keys().len();

        hand.combinaton = match combination_len {
            1 => 7,
            2 => {
                if cards_nominals.values().any(|val| *val == 4) {
                    if js == 1 || js == 4 {
                        7
                    } else {
                        6
                    }
                } else if js == 2 || js == 3 {
                    7
                } else {
                    5
                }
            }
            3 => {
                // KKK Q T
                if cards_nominals.values().any(|val| *val == 3) {
                    if js == 1 || js == 3 {
                        6
                    } else {
                        4
                    }
                } else {
                    // KK TT A
                    if js == 1 {
                        5
                    } else if js == 2 {
                        6
                    } else {
                        3
                    }
                }
            }
            // 234JJ || 2345J || 23456
            4 => {
                if js == 1 || js == 2 {
                    4
                } else {
                    2
                }
            }
            _ => {
                if js == 1 {
                    2
                } else {
                    1
                }
            }
        }
    }
    hands.sort();
    let mut res = 0;
    for (i, hand) in hands.iter().enumerate() {
        res += hand.bid * (i32::try_from(i).unwrap() + 1);
    }

    println!("{:?}", res);
}

//
// KK JJJ
//

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
