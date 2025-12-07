use std::fs::read_to_string;

use crate::Part;

pub fn run(input_path: &str, part: Part) {
    let contents = read_to_string(input_path).expect("Unable to open file");

    let lines: Vec<String> = contents.lines().map(String::from).collect();

    let mut nums: Vec<Vec<u64>> = Vec::new();
    for line in lines.iter().take(lines.len() - 1) {
        nums.push(
            line.split_whitespace()
                .map(|x| x.parse::<u64>().expect("Unable to parse as int"))
                .collect(),
        );
    }

    let ops = lines.last().unwrap();

    let num_cols = nums.first().unwrap().len();
    let mut answer: u64 = 0;
    for (col, op) in std::iter::zip(0..num_cols, ops.split_whitespace()) {
        let column = nums.iter().map(|row| &row[col]);

        answer += {
            let delta = match op {
                "*" => column.product::<u64>(),
                "+" => column.sum::<u64>(),
                _ => unreachable!("Unrecognized Operation"),
            };
            delta
        }
    }

    println!("Answer: {answer}");
}
