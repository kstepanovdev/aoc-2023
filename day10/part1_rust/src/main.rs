use std::fs::read_to_string;

#[derive(Debug, Default, Clone)]
struct Coord {
    pipe: char,
    row: usize,
    col: usize,
}

impl Coord {
    fn new(pipe: char, row: usize, col: usize) -> Self {
        Self { pipe, row, col }
    }
}

fn main() {
    let binding = read_to_string("../input.txt").unwrap();
    let input: Vec<Vec<char>> = binding
        .split('\n')
        .map(|splitted| splitted.chars().collect())
        .collect();
    let mut coords = vec![vec![Coord::default(); input[0].len()]; input.len()];
    let mut steps = 0;
    let mut start = Coord::default();

    for row in 0..coords.len() {
        for col in 0..coords[0].len() {
            let coord = Coord::new(input[row][col], row, col);
            coords[row][col] = coord.clone();
            if coord.pipe == 'S' {
                start = coord;
            }
        }
    }
    let mut remained = starting_neighbours(&start, &input);
    let mut visited = vec![vec![false; input[0].len()]; input.len()];
    visited[start.row][start.col] = true;

    bfs(input, &mut remained, &mut steps, &mut visited);

    println!("{:?}", steps / 2);
}

fn bfs(
    input: Vec<Vec<char>>,
    remained: &mut Vec<Coord>,
    steps: &mut usize,
    visited: &mut [Vec<bool>],
) {
    while let Some(coord) = remained.pop() {
        visited[coord.row][coord.col] = true;

        if let (Some(x_node), Some(y_node)) = next_point(&input, &coord) {
            let row_x = x_node.0;
            let col_x = x_node.1;
            let pipe_x = input[row_x][col_x];

            let row_y = y_node.0;
            let col_y = y_node.1;
            let pipe_y = input[row_y][col_y];

            if !visited[row_x][col_x] {
                remained.push(Coord::new(pipe_x, row_x, col_x));
            } else if !visited[row_y][col_y] {
                remained.push(Coord::new(pipe_y, row_y, col_y));
            }

            *steps += 1;
        }
    }
}

fn next_point(
    input: &Vec<Vec<char>>,
    coord: &Coord,
) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
    let row = coord.row;
    let col = coord.col;
    let right_bound = input[row].len();
    let bottom_bound = input.len();

    let right_node = if col + 1 < right_bound {
        Some((row, col + 1))
    } else {
        None
    };
    let left_node = if col > 0 { Some((row, col - 1)) } else { None };
    let bottom_node = if row + 1 < bottom_bound {
        Some((row + 1, col))
    } else {
        None
    };
    let upper_node = if row > 0 { Some((row - 1, col)) } else { None };

    match coord.pipe {
        '.' | 'S' => (None, None),
        'F' => (bottom_node, right_node),
        '-' => (left_node, right_node),
        '7' => (left_node, bottom_node),
        '|' => (upper_node, bottom_node),
        'L' => (upper_node, right_node),
        'J' => (upper_node, left_node),
        _ => unreachable!(),
    }
}

fn starting_neighbours(coord: &Coord, input: &Vec<Vec<char>>) -> Vec<Coord> {
    let mut remained = vec![];

    // left
    if coord.col > 0 {
        let col = coord.col - 1;
        if ['-', 'L', 'F'].contains(&input[coord.row][col]) {
            remained.push(Coord::new(input[coord.row][col], coord.row, col));
        }
    }
    // bottom
    if coord.row + 1 < input.len() {
        let row = coord.row + 1;
        if ['7', '|', 'F'].contains(&input[row][coord.col]) {
            remained.push(Coord::new(input[row][coord.col], row, coord.col));
        }
    }
    // right
    if coord.col + 1 < input.len() {
        let col = coord.col + 1;
        if ['-', 'J', '7'].contains(&input[coord.row][col]) {
            remained.push(Coord::new(input[coord.row][col], coord.row, col));
        }
    }
    // up
    if coord.row > 0 {
        let row = coord.row - 1;
        if ['|', 'F', '7'].contains(&input[row][coord.col]) {
            remained.push(Coord::new(input[row][coord.col], row, coord.col));
        }
    }

    remained
}
