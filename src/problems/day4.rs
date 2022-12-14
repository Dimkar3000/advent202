use crate::Problem;

#[derive(Clone, Copy)]
struct Elfe(u32, u32);

impl Elfe {
    fn new(input: &str) -> Self {
        let (f, s): (u32, u32) = input
            .split_once('-')
            .map(|(f, s)| (f.parse().unwrap(), s.parse().unwrap()))
            .unwrap();

        Self(f, s)
    }

    fn is_fully_inside(&self, other: &Elfe) -> bool {
        self.0 >= other.0 && self.1 <= other.1
    }

    fn overlaps(&self, other: &Elfe) -> bool {
        self.1 >= other.0 && self.0 <= other.1
    }
}

#[derive(Default)]
pub struct Problem4 {
    pairs: Vec<(Elfe, Elfe)>,
}

impl Problem4 {
    pub fn new() -> Self {
        Self { pairs: Vec::new() }
    }

    fn skeleton(&self, callback: &dyn Fn((Elfe, Elfe)) -> bool) -> String {
        self.pairs
            .iter()
            .filter(|x| callback(**x))
            .count()
            .to_string()
    }

    fn parse_line(&mut self, input: &str) {
        let result = input
            .split_once(',')
            .map(|(p1, p2)| (Elfe::new(p1), Elfe::new(p2)))
            .unwrap();
        self.pairs.push(result);
    }
}

impl Problem for Problem4 {
    fn part1(&mut self, input: &str) -> String {
        input.lines().for_each(|line| self.parse_line(line));
        let callback = |(p1, p2): (Elfe, Elfe)| p1.is_fully_inside(&p2) || p2.is_fully_inside(&p1);
        self.skeleton(&callback)
    }

    fn part2(&mut self, _input: &str) -> String {
        let callback = |(p1, p2): (Elfe, Elfe)| p1.overlaps(&p2) || p2.overlaps(&p1);
        self.skeleton(&callback)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        let input = "2-4,6-8
      2-3,4-5
      5-7,7-9
      2-8,3-7
      6-6,4-6
      2-6,4-8";
    }
}
