use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Default)]
struct Pair {
    first: Option<u32>,
    last: Option<u32>,
}
impl Pair {
    fn sum(&self) -> u32 {
        self.first.unwrap() * 10 + self.last.unwrap()
    }

    fn insert(&mut self, c: Option<char>) {
        if let Some(c) = c {
            let c = c.to_digit(10).unwrap();
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
    let mut result = 0;

    if let Ok(lines) = read_lines("../input.txt") {
        for line in lines.flatten() {
            let mut subres = Pair::default();

            for c in line.chars() {
                if c.is_numeric() {
                    subres.insert(Some(c));
                }
            }
            subres.insert(None);

            result += subres.sum();
        }
    }

    println!("{result}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
