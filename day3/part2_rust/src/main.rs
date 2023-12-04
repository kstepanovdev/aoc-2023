use std::{
    fs::File,
    io::{self, BufRead},
    path::Path, collections::HashMap, vec
};

#[derive(Debug, Clone)]
struct Candidate {
    number: String,
    i: usize,
    j_start: Option<usize>,
    j_end: Option<usize>,
}

impl Candidate {
    fn new(i: usize) -> Self {
        Self {
            number: String::new(),
            i,
            j_start: None,
            j_end: None,
        }
    }
}

fn main() {
    let mut result = 0;
    let mut gears_grid = HashMap::new();
    let mut rows_len = 0;
    let mut cols_len = 0;
    let mut candidates: Vec<Candidate> = vec![];

    if let Ok(lines) = read_lines("../input.txt") {
        for (i, line) in lines.flatten().enumerate() {
            cols_len = line.len();
            let mut candidate = Candidate::new(i);
            for (j, c) in line.chars().enumerate() {
                if c.is_numeric() {
                    candidate.number.push(c);
                    if candidate.j_start.is_none() {
                        candidate.j_start = Some(j);
                    }
                    candidate.j_end = Some(j);
                } else {
                    if !candidate.number.is_empty() {
                        candidates.push(candidate.clone());
                        candidate = Candidate::new(i);
                    }
                    if c == '*' {
                        gears_grid.insert((i, j), vec![]);
                    }
                }
            }
            if !candidate.number.is_empty() {
                candidates.push(candidate);
            }
            rows_len += 1;
        }
    }

    for candidate in candidates {
        check_adjacement(&candidate, &mut gears_grid, rows_len, cols_len);
    }

    gears_grid.values().filter(|el| el.len() == 2).for_each(|gear| result += gear[0] * gear[1]);

    println!("{}", result);
}

fn left_bound(i: usize) -> usize {
    0.max(i as i32 - 1) as usize
}

fn right_bound(i: usize, max_bound: usize) -> usize {
    max_bound.min(i + 1)
}

fn check_adjacement(candidate: &Candidate, gears_grid: &mut HashMap<(usize, usize), Vec<usize>>, rows_len: usize, cols_len: usize) {
    let number = candidate.number.parse::<usize>().unwrap();

    for i in left_bound(candidate.i)..=right_bound(candidate.i, rows_len) {
        for j in left_bound(candidate.j_start.unwrap())..=right_bound(candidate.j_end.unwrap(), cols_len) {
            if let Some(gear) = gears_grid.get_mut(&(i, j)) {
                gear.push(number);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
