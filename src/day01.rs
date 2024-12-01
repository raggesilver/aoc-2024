use std::collections::HashMap;

use crate::Day;

pub struct Day01;

fn parse_numbers(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for (index, number_str) in input.split_whitespace().enumerate() {
        let number = number_str.parse::<i32>().unwrap();
        if index % 2 == 0 {
            left.push(number);
        } else {
            right.push(number);
        }
    }

    (left, right)
}

impl Day for Day01 {
    fn part_one(&self, input: &str) {
        let (mut left, mut right) = parse_numbers(input);

        left.sort();
        right.sort();

        let total_sum: i32 = left
            .iter()
            .zip(right.iter())
            .map(|(&a, &b)| (a - b).abs())
            .collect::<Vec<i32>>()
            .iter()
            .sum();

        println!("Day 01, Part 1: {}", total_sum);
    }

    fn part_two(&self, input: &str) {
        let (left, right) = parse_numbers(input);

        let mut right_nums: HashMap<i32, i32> = HashMap::new();

        for &num in right.iter() {
            if right_nums.contains_key(&num) {
                let count = right_nums.get(&num).unwrap();
                right_nums.insert(num, count + 1);
            } else {
                right_nums.insert(num, 1);
            }
        }

        let similarity_score: i32 = left
            .iter()
            .map(|num| {
                let occurences = right_nums.get(num).unwrap_or(&0);
                num * occurences
            })
            .sum();

        println!("Day 01, Part 2: {}", similarity_score);
    }
}
