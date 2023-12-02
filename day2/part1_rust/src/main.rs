use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

const R_STOCK: i32 = 12;
const G_STOCK: i32 = 13;
const B_STOCK: i32 = 14;

fn main() {
    let mut result = 0;

    if let Ok(lines) = read_lines("../input.txt") {
        'loopa: for (i, line) in lines.flatten().enumerate() {
            let semi_parsed_line = line
                .split(':')
                .nth(1)
                .map(|s| s.split(&[';', ','][..]))
                .map_or(Vec::new(), |split_str| {
                    split_str
                        .flat_map(str::split_whitespace)
                        .map(str::trim)
                        .collect::<Vec<&str>>()
                        .chunks_exact(2)
                        .map(|chunk| {
                            let tuple: (i32, &str) = (
                                chunk.first().map_or(0, |&s| s.parse::<i32>().unwrap()),
                                chunk.get(1).map_or("", |&s| s),
                            );
                            tuple
                        })
                        .collect::<Vec<(i32, &str)>>()
                });

            for parsed in semi_parsed_line {
                match parsed {
                    (num, "red") => {
                        if num > R_STOCK {
                            continue 'loopa;
                        }
                    }
                    (num, "green") => {
                        if num > G_STOCK {
                            continue 'loopa;
                        }
                    }
                    (num, "blue") => {
                        if num > B_STOCK {
                            continue 'loopa;
                        }
                    }
                    _ => unreachable!(),
                }
            }

            result += i + 1;
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
