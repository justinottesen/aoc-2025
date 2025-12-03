use std::env;

fn main() {
    let usage = "Usage: aoc-2025 <day> [part]";

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

    println!("Executing with day={day}, part={part}");

    match (day, part) {
        _ => println!("Invalid day / part provided"),
    }
}
