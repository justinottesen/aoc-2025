use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run_part1(input_path: String) {
    let file = File::open(input_path).expect("Unable to open file");

    let reader = BufReader::new(file);

    let mut code = 0;

    let mut val = 50;
    for line_result in reader.lines() {
        let line = line_result.expect("Unable to parse line");

        let (dir_str, diff_str) = line.split_at(1);

        let dir: char = dir_str.parse().expect("Not a valid char");
        let diff: i32 = diff_str.parse().expect("not a valid integer");

        match dir {
            'L' => val -= diff,
            'R' => val += diff,
            _ => unreachable!("Inavlid direction found: {dir} (from line {line})"),
        }
        val %= 100;

        if val == 0 {
            code += 1;
        }
    }

    println!("Code: {code}");
}
