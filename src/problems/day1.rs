use std::cmp::{max, Reverse};

use crate::Problem;

#[derive(Default)]
pub struct Problem1 {
    elves: Vec<u32>,
}

impl Problem1 {
    pub fn new() -> Box<Self> {
        Box::new(Self::default())
    }
}

impl Problem for Problem1 {
    fn part1(&mut self, input: &str) -> String {
        let mut current_sum = 0;
        let mut current_max = 0;
        for line in input.lines() {
            match line.parse::<u32>() {
                Ok(calories) => {
                    current_sum += calories;
                }
                Err(_) => {
                    current_max = max(current_max, current_sum);
                    self.elves.push(current_sum);
                    current_sum = 0;
                }
            }
        }

        current_max.to_string()
    }

    fn part2(&mut self, _input: &str) -> String {
        self.elves.sort_by_key(|w| Reverse(*w));
        (self.elves[0] + self.elves[1] + self.elves[2]).to_string()
    }
}
