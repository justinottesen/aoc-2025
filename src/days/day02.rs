use crate::Part;

pub fn run(input_path: &str, part: Part) {
    let contents = std::fs::read_to_string(input_path).expect("Couldn't read file");

    let mut answer: u64 = 0;
    for range in contents.split(',') {
        let (low_str, high_str) = range.split_once('-').expect("Invalid range");
        println!("Low: {low_str}, High: {high_str}");

        let low: u64 = low_str.trim().parse().expect("Not a valid number");
        let high: u64 = high_str.trim().parse().expect("Not a valid number");

        for num in low..=high {
            let num_string = num.to_string();
            if num_string.len() % 2 != 0 {
                continue;
            }

            let (front, back) = num_string.split_at(num_string.len() / 2);
            if front == back {
                answer += num;
            }
        }
    }

    println!("Answer: {answer}");
}
