use crate::Day;
use std::collections::{HashMap, HashSet};

pub struct Day05;

#[allow(dead_code)]
fn print_manuals(manuals: &Vec<Vec<i32>>) {
    for manual in manuals {
        for m in manual {
            print!("{}, ", m);
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_must_be_before(must_be_before: &HashMap<i32, Vec<i32>>) {
    for (k, v) in must_be_before.iter() {
        print!("{} -> ", k);
        print!("{}", v[0]);
        for i in &v[1..] {
            print!(", {}", i);
        }
        println!();
    }
}

impl Day for Day05 {
    fn part_one(&self, input: &str) {
        let lines = input.lines();

        let mut second_part = false;
        let mut must_be_before: HashMap<i32, Vec<i32>> = HashMap::new();

        let mut manuals: Vec<Vec<i32>> = vec![];

        for line in lines {
            if line == "" {
                second_part = true;
                continue;
            }

            if second_part {
                manuals.push(
                    line.split(",")
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>(),
                );
            } else {
                let parts = line
                    .split("|")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();

                let (a, b) = (parts[0], parts[1]);

                if !must_be_before.contains_key(&a) {
                    must_be_before.insert(a, vec![]);
                }

                must_be_before.get_mut(&a).unwrap().push(b);
            }
        }

        let mut middle_page_sum: i32 = 0;

        'manual_loop: for manual in manuals {
            let mut _set: HashSet<i32> = HashSet::new();

            for page in manual.iter() {
                if must_be_before.contains_key(page) {
                    for rule in must_be_before.get(page).unwrap() {
                        if _set.contains(rule) {
                            // `page` should have been before `rule`, but since
                            // `rule` is already in the set, it means that
                            // `page` is after `rule` which is not allowed.

                            continue 'manual_loop;
                        }
                    }
                }
                _set.insert(*page);
            }

            let middle_page = manual[manual.len() / 2];
            middle_page_sum += middle_page;
        }

        println!("Day 05, Part 1: {}", middle_page_sum);
    }

    fn part_two(&self, _input: &str) {
        println!("Day 05, Part 2: {}", "todo");
    }
}
