use std::env;

fn main() {
    let usage = "Usage: aoc-2025 <day> <part> <input_path>";

    let mut args = env::args();

    let _ = args.next().expect("program name missing");

    let day: u8 = args
        .next()
        .expect(usage)
        .parse()
        .expect("day must be a number");

    let part: u8 = args
        .next()
        .expect(usage)
        .parse()
        .expect("part must be a number");

    let input_path: String = args.next().expect(usage);

    println!("Executing with day={day}, part={part}, input_path={input_path}");

    aoc_2025::run(day, part, input_path);
}
