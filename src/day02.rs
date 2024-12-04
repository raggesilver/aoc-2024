use crate::Day;

pub struct Day02;

fn is_report_safe(numbers: &Vec<i32>) -> bool {
    let initial_diff = numbers[0] - numbers[1];

    for i in 1..numbers.len() {
        let current_diff = numbers[i - 1] - numbers[i];
        let abs_diff = current_diff.abs();

        if initial_diff.signum() != current_diff.signum() || abs_diff > 3 || abs_diff < 1 {
            return false;
        }
    }

    true
}

// Not proud of this solution as it is very inefficient. I still don't
// understand why excluding only i - 1, i, and i + 1 when the error is found is
// not enough...
fn is_report_safe_with_tolerance(numbers: &Vec<i32>) -> bool {
    if is_report_safe(&numbers) {
        return true;
    }

    for i in 0..numbers.len() {
        let mut new_numbers = numbers.clone();
        new_numbers.remove(i);

        if is_report_safe(&new_numbers) {
            return true;
        }
    }

    false
}

impl Day for Day02 {
    fn part_one(&self, input: &str) {
        let mut safe_reports = 0;

        for line in input.lines() {
            let numbers: Vec<i32> = line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect();

            if is_report_safe(&numbers) {
                safe_reports += 1;
            }
        }

        println!("Day 02, Part 1: {}", safe_reports);
    }

    fn part_two(&self, input: &str) {
        let mut safe_reports = 0;

        for line in input.lines() {
            let numbers: Vec<i32> = line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect();

            if is_report_safe_with_tolerance(&numbers) {
                safe_reports += 1;
            }
        }

        println!("Day 02, Part 2: {}", safe_reports);
    }
}
