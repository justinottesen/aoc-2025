use crate::Part;

pub fn run(input_path: &str, part: Part) {
    let contents = std::fs::read_to_string(input_path).expect("Couldn't read file");

    let mut answer: u64 = 0;
    for range in contents.split(',') {
        let (low_str, high_str) = range.split_once('-').expect("Invalid range");

        let low: u64 = low_str.trim().parse().expect("Not a valid number");
        let high: u64 = high_str.trim().parse().expect("Not a valid number");

        for num in low..=high {
            let num_string = num.to_string();
            let bytes = num_string.as_bytes();

            let max_num_chunks = match part {
                Part::One => 2,
                Part::Two => bytes.len(),
            };

            for num_chunks in 2..=max_num_chunks {
                if bytes.len() % num_chunks != 0 {
                    continue;
                }

                let chunk_len = bytes.len() / num_chunks;

                let first = &bytes[..chunk_len];
                if bytes.chunks(chunk_len).all(|chunk| chunk == first) {
                    println!("Invalid: {num}");
                    answer += num;
                    break;
                }
            }
        }
    }

    println!("Answer: {answer}");
}
