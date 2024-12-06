use crate::Day;
use lazy_static::lazy_static;
use regex::Regex;

pub struct Day03;

lazy_static! {
    static ref MUL_REGEX: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
}

impl Day for Day03 {
    fn part_one(&self, input: &str) {
        let oprs = MUL_REGEX.captures_iter(input);

        let mut total: i32 = 0;

        for opr in oprs {
            let a = opr[1].parse::<i32>().unwrap();
            let b = opr[2].parse::<i32>().unwrap();
            total += a * b;
        }

        println!("Day 03, Part 1: {}", total);
    }

    fn part_two(&self, input: &str) {
        let mut valid = true;
        let mut total = 0;

        let mut i: usize = 0;
        while i < input.len() {
            if input.chars().nth(i) == Some('d') {
                if input[i..].starts_with("don't()") {
                    valid = false;
                    i += 7;
                    continue;
                } else if input[i..].starts_with("do()") {
                    valid = true;
                    i += 4;
                    continue;
                }
            } else if input.chars().nth(i) == Some('m') {
                if let Some(caps) = MUL_REGEX.captures(&input[i..i + 13]) {
                    let a = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let b = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();

                    if valid {
                        total += a * b;
                    }

                    i += caps.get(0).unwrap().as_str().len();
                    continue;
                }
            }
            i += 1;
        }

        println!("Day 03, Part 1: {}", total);
    }
}
