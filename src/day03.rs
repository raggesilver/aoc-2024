use crate::Day;
use lazy_static::lazy_static;
use regex::Regex;

pub struct Day03;

lazy_static! {
    static ref MUL_REGEX: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
}

impl Day for Day03 {
    fn part_one(&self, input: &str) {
        let operations = MUL_REGEX.captures_iter(input);

        let mut total: i32 = 0;

        for opr in operations {
            let (_, [a, b]) = opr.extract();
            let a: i32 = a.parse().unwrap();
            let b: i32 = b.parse().unwrap();
            total += a * b;
        }

        println!("Day 03, Part 1: {}", total);
    }

    fn part_two(&self, input: &str) {
        let mut valid = true;
        let mut total = 0;

        let mut i: usize = 0;
        while i < input.len() {
            match input.as_bytes()[i] {
                b'd' => {
                    if input[i..].starts_with("don't()") {
                        valid = false;
                        i += 7;
                        continue;
                    } else if input[i..].starts_with("do()") {
                        valid = true;
                        i += 4;
                        continue;
                    }
                }
                b'm' => {
                    if let Some(caps) = MUL_REGEX.captures(&input[i..i + 13]) {
                        let (full, [a, b]) = caps.extract();
                        let a: i32 = a.parse().unwrap();
                        let b: i32 = b.parse().unwrap();

                        if valid {
                            total += a * b;
                        }
                        i += full.len();
                        continue;
                    }
                }
                _ => {}
            }
            i += 1;
        }

        println!("Day 03, Part 2: {}", total);
    }
}
