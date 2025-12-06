use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::Part;

pub fn run(input_path: &str, part: Part) {
    let file = File::open(input_path).expect("Unable to open file");

    let reader = BufReader::new(file);

    let mut code = 0;

    let mut val = 50;
    for line_result in reader.lines() {
        let start = val;
        let line = line_result.expect("Unable to parse line");

        println!("Val is {val}, readling line {line}");

        let (dir_str, diff_str) = line.split_at(1);

        let dir: char = dir_str.parse().expect("Not a valid char");
        let diff: i32 = diff_str.parse().expect("not a valid integer");

        if part == Part::Two {
            // Count full rotations to get it between [0, 100)
            let rotations = diff / 100;
            if rotations > 0 {
                println!("More than a full rotation, incrementing code {code} {rotations}");
                code += rotations;
            }
        }

        // Perform rotation with remaining diff
        let rem_diff = diff % 100;
        match dir {
            'L' => val -= rem_diff,
            'R' => val += rem_diff,
            _ => unreachable!("Inavlid direction found: {dir} (from line {line})"),
        }

        let prev = val;
        val %= 100;
        if val < 0 {
            val += 100;
        }

        if part == Part::Two {
            // If different before / after mod, we passed 0 (and didn't end on it)
            if prev != val && val != 0 && start != 0 {
                println!("We passed 0, incrementing code {code} + 1");
                code += 1;
            }
        }

        // If we end on 0, increment
        if val == 0 {
            println!("Ends on a 0, incrementing code {code} + 1");
            code += 1;
        }
    }

    println!("Code: {code}");
}
