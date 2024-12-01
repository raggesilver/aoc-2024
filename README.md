# Advent of Code 2024

This repository contains my solutions for the [Advent of Code 2024](https://adventofcode.com/2024) challenges. I have chosen Rust as my language of choice for this year's challenges.

My Rust code is not the most idiomatic, but I am trying to improve it as I go along.

## Structure

Inside `src/` I have a file for each day. The file is named `dayXX.rs` where `XX` is the day number. Each file contains a struct that implements the `Day` trait — `Day` contains `part_one` and `part_two`, both of which take `&str` as input and return a nothing.

The `main.rs` file contains the logic to run the solutions for each day.

## Running

To run the solutions for a specific day, you can use the following command:

```sh
cargo run --release -- <day>
```

The program will read the input from `input/dayXX.txt` and print the solutions for both parts of the challenge.
