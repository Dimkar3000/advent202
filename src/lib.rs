mod problems;
pub use problems::*;

pub trait Problem {
    fn part1(&mut self, input: &str) -> String;
    fn part2(&mut self, input: &str) -> String;
}
