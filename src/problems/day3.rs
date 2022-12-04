use std::collections::{HashMap, HashSet};

use crate::Problem;

#[derive(Default)]
pub struct Problem3;

impl Problem3 {
    pub fn new() -> Box<Self> {
        Box::new(Self::default())
    }

    fn process_group(input: &str) -> u32 {
        let inputs = input
            .lines()
            .map(|x| x.chars().collect::<HashSet<char>>())
            .collect::<Vec<_>>();
        let common12: HashSet<char> = inputs[0].intersection(&inputs[1]).cloned().collect();
        inputs[2]
            .intersection(&common12)
            .cloned()
            .map(char_to_priority)
            .sum::<u32>()
    }

    fn process_row(line: &str) -> u32 {
        let mut store = HashMap::new();
        for i in line.chars().take(line.len() / 2) {
            let _ = *store.entry(i).or_insert(1);
        }

        for i in line.chars().skip(line.len() / 2) {
            if store.get(&i).is_some() {
                return char_to_priority(i);
            }
        }
        0
    }
}

fn char_to_priority(i: char) -> u32 {
    let priority = match i {
        'a'..='z' => (i as u8) - b'a' + 1,
        'A'..='Z' => (i as u8) - b'A' + 27,
        _ => unreachable!(),
    };

    priority as u32
}

#[cfg(test)]
mod tests {
    use crate::{problems::day3::char_to_priority, Problem3};

    #[test]
    fn proccess_group() {
        let a = Problem3::process_group(
            "vJrwpWtwJgWrhcsFMMfFFhFp
      jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
      PmmdzqPrVvPwwTWBwg",
        );
        assert_eq!(a, 18);
    }

    #[test]
    fn proccess_group1() {
        let a = Problem3::process_group(
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
      ttgJtRGJQctTZtZT
      CrZsJsPPZsGzwwsLwLmpwMDw",
        );
        assert_eq!(a, 52);
    }

    #[test]
    fn it_works() {
        let a = char_to_priority('a');
        let a_cap = char_to_priority('A');
        assert_eq!(a, 1);
        assert_eq!(a_cap, 27);
    }

    #[test]
    fn line1() {
        let a = Problem3::process_row("vJrwpWtwJgWrhcsFMMfFFhFp");
        assert_eq!(a, 16);
    }
    #[test]
    fn line2() {
        let a = Problem3::process_row("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
        assert_eq!(a, 38);
    }
    #[test]
    fn line3() {
        let a = Problem3::process_row("PmmdzqPrVvPwwTWBwg");
        assert_eq!(a, 42);
    }
    #[test]
    fn line4() {
        let a = Problem3::process_row("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn");
        assert_eq!(a, 22);
    }
    #[test]
    fn line5() {
        let a = Problem3::process_row("ttgJtRGJQctTZtZT");
        assert_eq!(a, 20);
    }

    #[test]
    fn line6() {
        let a = Problem3::process_row("CrZsJsPPZsGzwwsLwLmpwMDw");
        assert_eq!(a, 19);
    }
}

impl Problem for Problem3 {
    fn part1(&mut self, input: &str) -> String {
        input
            .lines()
            .map(Problem3::process_row)
            .sum::<u32>()
            .to_string()
    }

    fn part2(&mut self, input: &str) -> String {
        input
            .lines()
            .collect::<Vec<&str>>()
            .chunks(3)
            .map(|x| Problem3::process_group(&x.join("\n")))
            .sum::<u32>()
            .to_string()
    }
}
