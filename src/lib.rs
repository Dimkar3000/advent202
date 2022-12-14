mod problems;

pub use problems::*;

pub trait Problem: Sync {
    fn part1(&mut self, input: &str) -> String;
    fn part2(&mut self, input: &str) -> String;
}
