use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::Part;

const NEIGHBOR_DELTAS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn run(input_path: &str, part: Part) {
    let file = File::open(input_path).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut grid = {
        let mut mut_vec: Vec<Vec<u8>> = Vec::new();
        for line_result in reader.lines() {
            mut_vec.push(Vec::from(line_result.expect("Unable to parse line").trim()));
        }
        mut_vec
    };

    let mut answer: u32 = 0;
    loop {
        let mut new_grid: Vec<Vec<u8>> = Vec::new();
        let mut rolls_removed: u32 = 0;
        for (y, row) in grid.iter().enumerate() {
            let mut new_row: Vec<u8> = Vec::new();
            for (x, cell) in row.iter().enumerate() {
                if *cell != b'@' {
                    new_row.push(b'.');
                    continue;
                }
                let mut neighbors = 0;
                for (dx, dy) in NEIGHBOR_DELTAS {
                    let (ny, overflowy) = y.overflowing_add_signed(dy);
                    let (nx, overflowx) = x.overflowing_add_signed(dx);
                    if overflowx || overflowy || ny >= grid.len() || nx >= row.len() {
                        continue;
                    }
                    if grid[ny][nx] == b'@' {
                        neighbors += 1;
                    }
                }

                if neighbors < 4 {
                    new_row.push(b'x');
                    rolls_removed += 1;
                } else {
                    new_row.push(b'@');
                }
            }
            new_grid.push(new_row);
        }

        if rolls_removed == 0 {
            break;
        } else {
            answer += rolls_removed;
        }

        if part == Part::One {
            break;
        }

        grid = new_grid;
    }

    println!("Answer: {answer}");
}
