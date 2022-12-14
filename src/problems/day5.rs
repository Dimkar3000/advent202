use crate::Problem;

#[derive(Debug, Clone, Copy)]
struct Command(usize, usize, usize);

impl Command {
    fn from_line(line: &str) -> Self {
        // println!("\"{line}\"");
        let items: Vec<_> = line.trim().split(' ').collect();
        let how_many: usize = items[1].parse().unwrap();
        let from: usize = items[3].parse().unwrap();
        let to: usize = items[5].parse().unwrap();
        Self(how_many, from - 1, to - 1)
    }
}

#[derive(Debug, Clone, Copy)]
struct Crate(char);

impl Crate {
    fn new(input: &str) -> Option<Self> {
        let input = input.trim();
        if let Some(c) = input.chars().nth(1) {
            return Some(Self(c));
        }
        None
    }
}

#[derive(Default, Debug)]
pub struct Problem5 {
    stacks: Vec<Vec<Crate>>,
    commands: Vec<Command>,
}

impl Problem5 {
    pub fn new() -> Self {
        Self::default()
    }

    fn process_state(&mut self, line: &str) {
        let items_slices = line.chars().collect::<Vec<_>>();
        let items: Vec<String> = items_slices
            .chunks(4)
            .map(|x| x.iter().collect::<String>())
            .collect();
        for (i, item) in items.iter().enumerate() {
            if self.stacks.get(i).is_none() {
                self.stacks.push(Vec::new())
            }
            let crt = Crate::new(item);
            if crt.is_none() {
                continue;
            }
            self.stacks[i].insert(0, crt.unwrap());
        }
    }

    fn proccess_input(&mut self, input: &str) {
        self.commands = Vec::new();
        self.stacks = Vec::new();

        let mut part1done = false;

        for line in input.lines() {
            if line.starts_with(" 1") {
                continue;
            }
            if line.trim().is_empty() {
                part1done = true;
                continue;
            }

            if !part1done {
                self.process_state(line);
            } else {
                let command = Command::from_line(line.trim());
                self.commands.push(command);
            }
        }
    }
}

impl Problem for Problem5 {
    fn part1(&mut self, input: &str) -> String {
        self.proccess_input(input);

        for c in &mut self.commands {
            let (how_many, from, to) = (c.0, c.1, c.2);
            for _ in 0..how_many {
                let i = self.stacks[from].pop().unwrap();
                self.stacks[to].push(i);
            }
        }

        self.stacks.iter().map(|s| s.last().unwrap().0).collect()
    }

    fn part2(&mut self, input: &str) -> String {
        self.proccess_input(input);

        for c in &mut self.commands {
            let (how_many, from, to) = (c.0, c.1, c.2);

            for i in 0..how_many {
                let index = self.stacks[from].len() + i - how_many;
                let c = self.stacks[from].remove(index);
                self.stacks[to].push(c);
            }
        }

        self.stacks.iter().map(|s| s.last().unwrap().0).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Problem, Problem5};

    #[test]
    fn proccess_group() {
        let mut problem = Problem5::new();
        let result = problem.part1(
            r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#,
        );

        assert_eq!(result, "CMZ");
    }

    #[test]

    fn proccess_group2() {
        let mut problem = Problem5::new();
        let result = problem.part2(
            r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#,
        );

        assert_eq!(result, "MCD");
    }
}
