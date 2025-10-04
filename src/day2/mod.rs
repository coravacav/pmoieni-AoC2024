use std::{
    cmp::Ordering,
    fs,
    io::{BufReader, Read},
};

use crate::day::Solution;

pub struct Day2 {
    name: &'static str,
}

impl Day2 {
    pub fn new() -> Day2 {
        Day2 { name: "two" }
    }
}

enum State {
    Increasing,
    Decreasing,
    Neutral,
}

impl Solution for Day2 {
    fn get_day(&self) -> &'static str {
        self.name
    }
    fn result_p1(&self) -> String {
        let file = fs::File::open("src/day2/input_p1.txt").expect("file didn't open");

        let reader = BufReader::new(file);
        let mut state = State::Neutral;
        let mut last_digit = None;
        let mut safe_count = 0;
        let mut current_number: Option<i32> = None;

        for character in reader.bytes() {
            match character {
                Ok(b' ') | Ok(b'\n') => {
                    if let Some(parsed) = current_number {
                        if last_digit.is_none() {
                            last_digit = Some(parsed);
                            current_number = None;
                            continue;
                        }

                        state = match (last_digit.cmp(&Some(parsed)), &state) {
                            (Ordering::Greater, State::Neutral) => State::Decreasing,
                            (Ordering::Greater, State::Increasing) => break,
                            (Ordering::Less, State::Neutral) => State::Increasing,
                            (Ordering::Less, State::Decreasing) => break,
                            (Ordering::Equal, State::Increasing | State::Decreasing) => break,
                            _ => state,
                        };

                        safe_count += 1;
                        current_number = None;
                    }
                }
                Ok(b) if b.is_ascii_digit() => {
                    let digit = (b - b'0') as i32;
                    current_number = Some(current_number.unwrap_or(0) * 10 + digit);
                }
                Err(e) => panic!("error reading byte: {}", e),
                _ => {}
            }
        }

        safe_count.to_string()
    }
}
