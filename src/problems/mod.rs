use crate::Problem;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub use day1::*;
pub use day10::*;
pub use day11::*;
pub use day12::*;
pub use day13::*;
pub use day14::*;
pub use day15::*;
pub use day2::*;
pub use day3::*;
pub use day4::*;
pub use day5::*;
pub use day6::*;
pub use day7::*;
pub use day8::*;
pub use day9::*;

pub fn get_problem() -> Vec<Box<dyn Problem>> {
    let v: Vec<Box<dyn Problem>> = vec![
        Box::new(Problem1::new()),
        Box::new(Problem2::new()),
        Box::new(Problem3::new()),
        Box::new(Problem4::new()),
        Box::new(Problem5::new()),
        Box::new(Problem6::new()),
        Box::new(Problem7::new()),
        Box::new(Problem8::new()),
        Box::new(Problem9::new()),
        Box::new(Problem10::new()),
        Box::new(Problem11::new()),
        Box::new(Problem12::new()),
        Box::new(Problem13::new()),
        Box::new(Problem14::new()),
        Box::new(Problem15::new()),
    ];
    v
}
