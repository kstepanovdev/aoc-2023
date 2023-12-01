use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Default)]
struct Pair {
    first: Option<i32>,
    last: Option<i32>,
}
impl Pair {
    fn sum(&self) -> i32 {
        if self.first.is_some() {
            if self.last.is_some() {
                self.first.unwrap() * 10 + self.last.unwrap()
            } else {
                self.first.unwrap() * 10 + self.first.unwrap()
            }
        } else {
            0
        }
    }

    fn insert(&mut self, c: Option<i32>) {
        if let Some(c) = c {
            if self.first.is_none() {
                self.first = Some(c);
            } else {
                self.last = Some(c);
            }
        } else if self.last.is_none() {
            self.last = self.first;
        }
    }
}

fn main() {
    let hm = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut result = 0;

    if let Ok(lines) = read_lines("../input.txt") {
        for line in lines.flatten() {
            let mut pair = Pair::default();
            for i in 0..line.len() {
                pair.insert(rec_search(&line[i..], &hm));
            }

            result += pair.sum();
        }
    }

    println!("{result}");
}

fn rec_search(substr: &str, hm: &HashMap<&str, i32>) -> Option<i32> {
    for c in substr.chars() {
        if c.is_numeric() {
            let c = c.to_digit(10).unwrap().try_into().unwrap();
            if c > 0 {
                return Some(c);
            };
        } else {
            break;
        }
    }

    for (k, v) in hm {
        if substr.starts_with(k) {
            return Some(*v);
        }
    }

    None
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
