use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::Part;

enum InputSection {
    Range,
    Id,
}
pub fn run(input_path: &str, part: Part) {
    let file = File::open(input_path).expect("Unable to open file");

    let reader = BufReader::new(file);

    let mut input_section = InputSection::Range;

    let mut ranges: Vec<(u64, u64)> = Vec::new();

    let mut answer: u64 = 0;
    for line_result in reader.lines() {
        let line_str = line_result.expect("Unable to unwrap line");
        let line_str = line_str.trim();
        if line_str.is_empty() {
            input_section = InputSection::Id;
            simplify_ranges(&mut ranges);

            match part {
                Part::One => continue,
                Part::Two => {
                    for (low, high) in &ranges {
                        answer += high - low + 1;
                    }
                    break;
                }
            }
        }

        match input_section {
            InputSection::Range => {
                let (low_str, high_str) = line_str
                    .trim()
                    .split_once('-')
                    .expect("Couldn't parse range");
                ranges.push((
                    low_str.parse::<u64>().expect("Couldn't parse int from low"),
                    high_str
                        .parse::<u64>()
                        .expect("Couldn't parse int from high"),
                ));
            }
            InputSection::Id => {
                for (low, high) in &ranges {
                    let val = line_str
                        .parse::<u64>()
                        .expect("Unable to parse int from line");
                    if *low <= val && val <= *high {
                        answer += 1;
                        break;
                    }
                }
            }
        }
    }

    println!("Answer: {answer}");
}

fn simplify_ranges(ranges: &mut Vec<(u64, u64)>) {
    ranges.sort();
    let mut out: Vec<(u64, u64)> = Vec::new();
    for (low, high) in ranges.iter() {
        if out.len() == 0 {
            out.push((*low, *high));
        } else {
            let last = out.last_mut().unwrap();
            if *low <= last.1 {
                *last = (last.0, std::cmp::max(last.1, *high));
            } else {
                out.push((*low, *high));
            }
        }
    }
    *ranges = out;
}
