use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::Part;

pub fn run(input_path: &str, part: Part) {
    let file = File::open(input_path).expect("Unable to open file");

    let reader = BufReader::new(file);

    let mut answer = 0;
    for line_result in reader.lines() {
        let line = line_result.expect("Unable to parse line");
        let line = line.trim();

        let mut line_itr = line.chars().rev();

        let mut ls = line_itr.next().expect("Line too short");
        let mut ms = line_itr.next().expect("Line too short");

        for digit in line_itr {
            if digit >= ms {
                if ls <= ms {
                    ls = ms;
                }
                ms = digit;
            }
        }

        let ls_value = ls.to_digit(10).expect("Couldn't convert char to digit");
        let ms_value = ms.to_digit(10).expect("Couldn't convert char to digit");

        let value = ms_value * 10 + ls_value;
        answer += value;

        println!("Line: {line}, value {value}");
    }

    println!("Answer: {answer}")
}
