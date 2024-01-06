use std::fs;

fn main() {
    let binding = fs::read_to_string("../input.txt").unwrap();
    let maps: Vec<Vec<Vec<char>>> = binding
        .split("\n\n")
        .map(|map| {
            map.split('\n')
                .map(|m| m.chars().collect::<Vec<char>>())
                .collect()
        })
        .collect();

    let mut result = 0;
    for map in maps {
        let v = find_reflection(&map);
        if let Some(val) = v {
            result += val;
        }

        let h = find_reflection(&transpose_matrix(&map));
        if let Some(val) = h {
            result += val * 100;
        }
    }

    println!("{}", result);
}

fn find_reflection(map: &Vec<Vec<char>>) -> Option<usize> {
    for col in 0..map[0].len() {
        let offset_slots = (col + 1).min(map[0].len() - col - 1);

        let mut reflected = true;
        let mut calculated = false;

        for row in map {
            for offset in 0..offset_slots {
                calculated = true;
                if row[col - offset] != row[col + offset + 1] {
                    reflected = false;
                    break;
                }
            }
        }

        if reflected && calculated {
            return Some(col + 1);
        }
    }

    None
}

fn transpose_matrix(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut result = vec![vec![' '; rows]; cols];

    for (i, row) in matrix.iter().enumerate().take(rows) {
        for (j, col) in result.iter_mut().enumerate().take(cols) {
            col[i] = row[j];
        }
    }

    result
}
