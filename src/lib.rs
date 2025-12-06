mod days;

pub fn run(day: u8, part: u8, input_path: String) {
    let part_enum = match part {
        1 => Part::One,
        2 => Part::Two,
        _ => unreachable!("Encountered unexpected part number: {part}"),
    };
    match day {
        1 => days::day01::run(input_path, part_enum),
        _ => eprintln!("Day {day}, part {part} not implemented"),
    }
}

#[derive(PartialEq)]
pub enum Part {
    One,
    Two,
}
