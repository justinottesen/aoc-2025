mod days;

pub fn run(day: u8, part: u8, input_path: String) {
    match (day, part) {
        (1, 1) => days::day01::run_part1(input_path),
        _ => eprintln!("Day {day}, part {part} not implemented"),
    }
}
