use crate::Problem;

#[derive(Clone, Copy)]
enum RPC {
    Rock,
    Paper,
    Scissor,
}
impl RPC {
    fn what_to_do(self, c: char) -> RPC {
        match (self, c) {
            (RPC::Rock, 'X') => RPC::Scissor,
            (RPC::Rock, 'Y') => RPC::Rock,
            (RPC::Rock, 'Z') => RPC::Paper,

            (RPC::Paper, 'X') => RPC::Rock,
            (RPC::Paper, 'Y') => RPC::Paper,
            (RPC::Paper, 'Z') => RPC::Scissor,

            (RPC::Scissor, 'X') => RPC::Paper,
            (RPC::Scissor, 'Y') => RPC::Scissor,
            (RPC::Scissor, 'Z') => RPC::Rock,

            _ => unreachable!(),
        }
    }

    fn points(&self) -> u32 {
        match self {
            RPC::Rock => 1,
            RPC::Paper => 2,
            RPC::Scissor => 3,
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            'A' | 'X' => RPC::Rock,
            'B' | 'Y' => RPC::Paper,
            'C' | 'Z' => RPC::Scissor,
            _ => unreachable!(),
        }
    }
}

#[derive(Default)]
pub struct Problem2;

impl Problem2 {
    pub fn new() -> Self {
        Self::default()
    }

    /**
     * Common skeleton code for both parts
     *
     */
    fn skeleton(input: &str, callback: &dyn Fn(Vec<&str>) -> (RPC, RPC)) -> String {
        input
            .lines()
            .map(|x| x.split(' ').collect::<Vec<&str>>())
            .map(callback)
            .map(|x| Problem2::game_score(x.1, x.0))
            .sum::<u32>()
            .to_string()
    }

    fn game_score(myself: RPC, enemy: RPC) -> u32 {
        let play = myself.points();
        let game = match (myself, enemy) {
            (RPC::Rock, RPC::Rock) => 3,
            (RPC::Rock, RPC::Paper) => 0,
            (RPC::Rock, RPC::Scissor) => 6,

            (RPC::Paper, RPC::Rock) => 6,
            (RPC::Paper, RPC::Paper) => 3,
            (RPC::Paper, RPC::Scissor) => 0,

            (RPC::Scissor, RPC::Rock) => 0,
            (RPC::Scissor, RPC::Paper) => 6,
            (RPC::Scissor, RPC::Scissor) => 3,
        };
        game + play
    }
}

impl Problem for Problem2 {
    fn part1(&mut self, input: &str) -> String {
        Problem2::skeleton(input, &|x| {
            (
                RPC::from_char(x[0].chars().next().unwrap()),
                RPC::from_char(x[1].chars().next().unwrap()),
            )
        })
    }

    fn part2(&mut self, input: &str) -> String {
        Problem2::skeleton(input, &|x| {
            let enemy = RPC::from_char(x[0].chars().next().unwrap());
            let me = enemy.what_to_do(x[1].chars().next().unwrap());
            (enemy, me)
        })
    }
}
