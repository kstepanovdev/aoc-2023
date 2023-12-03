use std::{
    cmp::min,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Debug, Clone)]
struct Candidate {
    number: String,
    i_start: usize,
    j_start: Option<usize>,
    j_end: Option<usize>,
}

impl Candidate {
    fn new(i_start: usize) -> Self {
        Self {
            number: String::new(),
            i_start,
            j_start: None,
            j_end: None,
        }
    }
}

fn main() {
    let mut result = 0;
    let mut symbols_grid = vec![];
    let mut candidates: Vec<Candidate> = vec![];

    if let Ok(lines) = read_lines("../input.txt") {
        for (i, line) in lines.flatten().enumerate() {
            symbols_grid.push(vec![false; line.len()]);
            let mut candidate = Candidate::new(left_bound(i));
            for (j, c) in line.chars().enumerate() {
                if c.is_numeric() {
                    candidate.number.push(c);
                    if candidate.j_start.is_none() {
                        candidate.j_start = Some(left_bound(j));
                    }
                    candidate.j_end = Some(right_bound(j, line.len()));
                } else {
                    if !candidate.number.is_empty() {
                        candidates.push(candidate.clone());
                        candidate = Candidate::new(left_bound(i));
                    }
                    if c != '.' {
                        symbols_grid[i][j] = true;
                    }
                }
            }
            if !candidate.number.is_empty() {
                candidates.push(candidate);
            }
        }
    }

    for candidate in candidates {
        result += check_adjacement(&candidate, &symbols_grid);
    }

    println!("{}", result);
}

fn left_bound(i: usize) -> usize {
    if i > 1 {
        i - 1
    } else if i > 2 {
        i - 2
    } else {
        0
    }
}

fn right_bound(i: usize, max_bound: usize) -> usize {
    min(max_bound, i + 2)
}

fn check_adjacement(candidate: &Candidate, symbols_grid: &Vec<Vec<bool>>) -> u32 {
    let Candidate {
        number,
        i_start,
        j_start,
        j_end,
    } = candidate;
    let j_start = j_start.unwrap();
    let j_end = j_end.unwrap();
    let i_end = right_bound(i_start + 1, symbols_grid.len());

    for line in symbols_grid.iter().take(i_end).skip(*i_start) {
        let slice = &line[j_start..j_end];
        if slice.contains(&true) {
            return number.parse::<u32>().unwrap();
        }
    }
    0
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
