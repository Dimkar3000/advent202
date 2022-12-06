use std::collections::HashSet;

use crate::Problem;

#[derive(Default, Debug)]
pub struct Problem6 {}

impl Problem6 {
    pub fn new() -> Box<Self> {
        Box::new(Self::default())
    }

    pub fn skeleton(input: &str, size: usize) -> String {
        let mut result: Option<String> = None;
        let mut current = String::new();
        input.chars().enumerate().any(|(index, next)| {
            current.push(next);
            if current.len() > size {
                current.remove(0);
            }

            if current.len() < size {
                return false;
            }

            if has_no_duplicate_letters(&current) {
                result = Some((index + 1).to_string());
                return true;
            }
            false
        });
        return result.unwrap();
    }
}

fn has_no_duplicate_letters(s: &str) -> bool {
    let mut set = HashSet::new();
    for c in s.chars() {
        if set.contains(&c) {
            return false;
        }
        set.insert(c);
    }
    true
}

impl Problem for Problem6 {
    fn part1(&mut self, input: &str) -> String {
        Problem6::skeleton(input, 4)
    }

    fn part2(&mut self, input: &str) -> String {
        Problem6::skeleton(input, 14)
    }
}
