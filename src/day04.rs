use crate::Day;

pub struct Day04;

impl Day for Day04 {
    fn part_one(&self, input: &str) {
        let lines = input.lines().collect::<Vec<&str>>();

        let mut count = 0;

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.bytes().enumerate() {
                if c != b'X' {
                    continue;
                }

                let directions: [(i32, i32); 8] = [
                    (-1, 0),  // left
                    (-1, -1), // top left
                    (0, -1),  // up
                    (1, -1),  // top right
                    (1, 0),   // right
                    (1, 1),   // bottom right
                    (0, 1),   // down
                    (-1, 1),  // bottom left
                ];

                for (dx, dy) in directions {
                    if x as i32 + 3 * dx < 0
                        || x as i32 + 3 * dx >= lines[y].len() as i32
                        || y as i32 + 3 * dy < 0
                        || y as i32 + 3 * dy >= lines.len() as i32
                    {
                        continue;
                    }

                    let word = (0..4)
                        .map(|i| {
                            let new_y = y as i32 + i * dy;
                            let new_x = x as i32 + i * dx;
                            // At this point we know the indices will be valid
                            lines[new_y as usize].as_bytes()[new_x as usize] as char
                        })
                        .collect::<String>();

                    if word == "XMAS" || word == "SAMX" {
                        count += 1;
                    }
                }
            }
        }

        println!("Day 04, Part 1: {}", count);
    }

    fn part_two(&self, input: &str) {
        let lines = input.lines().collect::<Vec<&str>>();
        let line_length = lines[0].len();

        let mut count = 0;

        for y in 1..lines.len() - 1 {
            for x in 1..line_length - 1 {
                if lines[y].as_bytes()[x] != b'A' {
                    continue;
                }

                let first_bytes = [
                    lines[y - 1].as_bytes()[x - 1],
                    lines[y + 1].as_bytes()[x + 1],
                ];
                let second_bytes = [
                    lines[y - 1].as_bytes()[x + 1],
                    lines[y + 1].as_bytes()[x - 1],
                ];

                if (first_bytes == [b'M', b'S'] || first_bytes == [b'S', b'M'])
                    && (second_bytes == [b'M', b'S'] || second_bytes == [b'S', b'M'])
                {
                    count += 1;
                }
            }
        }

        println!("Day 04, Part 2: {}", count);
    }
}
