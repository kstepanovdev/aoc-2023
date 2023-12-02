use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let mut result = 0;

    if let Ok(lines) = read_lines("../input.txt") {
        for line in lines.flatten() {
            let mut g_red = 0;
            let mut g_green = 0;
            let mut g_blue = 0;

            let semi_parsed_line = line
                .split(':')
                .nth(1)
                .map(|s| s.split(&[';', ','][..]))
                .map_or( Vec::new(), |split_str| {
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
                        g_red = num.max(g_red);
                    }
                    (num, "green") => {
                        g_green = num.max(g_green);
                    }
                    (num, "blue") => {
                        g_blue = num.max(g_blue);
                    }
                    _ => unreachable!(),
                }
            }

            result += g_red * g_green * g_blue;
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
