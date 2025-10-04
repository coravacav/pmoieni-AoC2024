use std::{
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
        let mut last_digit = -1;
        let mut safe_count = 0;

        for line in reader.lines() {
            let line = line.expect("no line");
            for digit in line.split_whitespace() {
                let parsed: i32 = digit.parse().expect("couldn't parse number");
                if last_digit == -1 {
                    last_digit = parsed;
                    continue;
                }

                if last_digit > parsed {
                    if matches!(state, State::Neutral) {
                        state = State::Decreasing;
                        continue;
                    }

                    if !matches!(state, State::Decreasing) {
                        state = State::Neutral;
                        last_digit = -1;
                        break;
                    }
                } else if last_digit < parsed {
                    if matches!(state, State::Neutral) {
                        state = State::Increasing;
                        continue;
                    }

                    if !matches!(state, State::Increasing) {
                        state = State::Neutral;
                        last_digit = -1;
                        break;
                    }
                } else if !matches!(state, State::Neutral) {
                    state = State::Neutral;
                    last_digit = -1;
                    break;
                }

                safe_count += 1;
            }
        }

        safe_count.to_string()
    }
}
