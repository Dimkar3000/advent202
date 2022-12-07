use crate::Problem;

#[derive(Default, Debug)]
pub struct Problem8;

impl Problem8 {
    pub fn new() -> Box<Self> {
        Box::new(Self::default())
    }
}

impl Problem for Problem8 {
    fn part1(&mut self, _input: &str) -> String {
        "todo!()".to_string()
    }

    fn part2(&mut self, _input: &str) -> String {
        "todo!()".to_string()
    }
}
