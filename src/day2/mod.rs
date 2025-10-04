use std::{
    cmp::Ordering,
    fs,
    io::{BufRead, BufReader},
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

        for line in reader.lines() {
            let line = line.expect("no line");
            for digit in line.split_whitespace() {
                let parsed: i32 = digit.parse().expect("couldn't parse number");
                if last_digit.is_none() {
                    last_digit = Some(parsed);
                    continue;
                }

                state = match (last_digit.cmp(&Some(parsed)), &state) {
                    (Ordering::Greater, State::Neutral) => State::Decreasing,
                    (Ordering::Greater, State::Increasing) => {
                        break;
                    }
                    (Ordering::Less, State::Neutral) => State::Increasing,
                    (Ordering::Less, State::Decreasing) => {
                        break;
                    }
                    (Ordering::Equal, State::Increasing | State::Decreasing) => {
                        break;
                    }
                    _ => state,
                };

                safe_count += 1;
            }
        }

        safe_count.to_string()
    }
}
