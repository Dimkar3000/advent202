use crate::Problem;

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
    Sqr,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operations: (Op, u64),
    test: u64,
    truth_monkey: usize,
    false_monkey: usize,
    activity: u64,
}

impl Monkey {
    fn from_input(input: &str) -> Self {
        let mut l = input.lines();

        l.next().unwrap();

        let starting_items = l
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split(',')
            .map(|x| x.trim().parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let operations = l
            .next()
            .unwrap()
            .split_once("= old ")
            .unwrap()
            .1
            .split_once(' ')
            .map(|(x0, x1)| match (x0, x1) {
                ("*", "old") => (Op::Sqr, 0),
                ("*", x1) => (Op::Mul, x1.parse().unwrap()),
                ("+", x1) => (Op::Add, x1.parse().unwrap()),
                _ => unreachable!(),
            })
            .unwrap();

        let test = l
            .next()
            .unwrap()
            .split_once("by ")
            .unwrap()
            .1
            .parse::<u64>()
            .unwrap();

        let truth_monkey = l
            .next()
            .unwrap()
            .split_once("monkey ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();

        let false_monkey = l
            .next()
            .unwrap()
            .split_once("monkey ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();

        Monkey {
            items: starting_items,
            operations,
            test,
            truth_monkey,
            false_monkey,
            activity: 0,
        }
    }
}

pub struct Problem11 {
    monkeys: Vec<Monkey>,
}

impl Problem11 {
    pub fn new() -> Self {
        Self {
            monkeys: Vec::new(),
        }
    }

    fn single_round(&mut self, worry: bool) {
        let lcm: u64 = self.monkeys.iter().map(|x| x.test).product();

        for i in 0..self.monkeys.len() {
            while !self.monkeys[i].items.is_empty() {
                self.monkeys[i].activity += 1;
                let item = self.monkeys[i].items.pop().unwrap();

                let mut value = calculate_new_level(
                    item,
                    &self.monkeys[i].operations.0,
                    self.monkeys[i].operations.1,
                );
                if worry {
                    value /= 3
                } else {
                    value %= lcm;
                }

                if value % self.monkeys[i].test == 0 {
                    let xx = self.monkeys[i].truth_monkey;
                    self.monkeys[xx].items.push(value);
                } else {
                    let xx = self.monkeys[i].false_monkey;
                    self.monkeys[xx].items.push(value);
                }
            }
        }
    }

    fn proccess_input(&mut self, input: &str) {
        self.monkeys.clear();
        let monkey_input = input.split("Monkey ");

        for minput in monkey_input {
            if minput.trim().is_empty() {
                continue;
            }
            self.monkeys.push(Monkey::from_input(minput));
        }
    }
}

fn calculate_new_level(item: u64, op: &Op, how_much: u64) -> u64 {
    match op {
        Op::Add => item + how_much,
        Op::Mul => item * how_much,
        Op::Sqr => item * item,
    }
}

impl Problem for Problem11 {
    fn part1(&mut self, input: &str) -> String {
        self.proccess_input(input);
        for _ in 0..20 {
            self.single_round(true);
        }
        let mut activities: Vec<u64> = self.monkeys.iter().map(|x| x.activity).collect();

        activities.sort();
        activities.reverse();

        format!("{}", activities[0] * activities[1])
    }

    fn part2(&mut self, input: &str) -> String {
        self.proccess_input(input);
        for _ in 0..10000 {
            self.single_round(false);
        }

        let mut activities: Vec<u64> = self.monkeys.iter().map(|x| x.activity).collect();

        activities.sort();
        activities.reverse();

        format!("{}", activities[0] * activities[1])
    }
}
