mod days;

type DayRunner = fn(&str, Part);

pub fn run(day: u8, part: u8, input_path: String) {
    let part_enum = match part {
        1 => Part::One,
        2 => Part::Two,
        _ => unreachable!("Encountered unexpected part number: {part}"),
    };

    let runner: Option<DayRunner> = match day {
        1 => Some(days::day01::run),
        2 => Some(days::day02::run),
        3 => Some(days::day03::run),
        _ => None,
    };

    runner.expect("Day not implemented")(&input_path, part_enum);
}

#[derive(PartialEq, Clone)]
pub enum Part {
    One,
    Two,
}
