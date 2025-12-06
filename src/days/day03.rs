use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::Part;

pub fn run(input_path: &str, part: Part) {
    let num_batteries = match part {
        Part::One => 2,
        Part::Two => 12,
    };

    let file = File::open(input_path).expect("Unable to open file");

    let reader = BufReader::new(file);

    let mut answer: u64 = 0;
    for line_result in reader.lines() {
        let line = line_result.expect("Unable to parse line");
        let line = line.trim();

        let joltage_str_len = line.len() - num_batteries;

        let mut joltage: Vec<u8> = line[joltage_str_len..].bytes().collect();

        let line_itr = line[..joltage_str_len].bytes().rev();

        for digit in line_itr {
            update_joltage(digit, &mut joltage[..]);
        }

        let joltage_str = std::str::from_utf8(&joltage).expect("Non-UTF8 digits in joltage");
        println!("Line: {line}, value {joltage_str}");

        answer += joltage_str.parse::<u64>().expect("Unable to parse joltage");
    }

    println!("Answer: {answer}")
}

pub fn update_joltage(digit: u8, joltage_str: &mut [u8]) {
    if joltage_str.len() == 0 {
        return;
    }

    if digit >= joltage_str[0] {
        update_joltage(joltage_str[0], &mut joltage_str[1..]);
        joltage_str[0] = digit;
    }
}
