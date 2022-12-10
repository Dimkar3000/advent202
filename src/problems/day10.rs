use crate::Problem;

const SCREEN_WIDTH: usize = 40;
const SCREEN_HEIGHT: usize = 6;

#[derive(Debug, Clone, Copy)]
enum Instr {
    Noop,
    ADDX(i32),
}

struct CPU {
    cycles: i32,
    partial_instruction: bool,
    register: i32,
    instructions: Vec<Instr>,
    screen: [char; SCREEN_WIDTH * SCREEN_HEIGHT],
}
impl CPU {
    pub fn new() -> Self {
        Self {
            cycles: 0,
            register: 1,
            partial_instruction: false,
            instructions: Vec::new(),
            screen: ['.'; SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }

    fn load_program(&mut self, input: &str) {
        for line in input.lines() {
            if line == "noop" {
                self.instructions.insert(0, Instr::Noop)
            } else {
                let (_, count) = line
                    .split_once(' ')
                    .map(|(x0, x1)| (x0, x1.trim().parse::<i32>().unwrap()))
                    .unwrap();
                self.instructions.insert(0, Instr::ADDX(count))
            }
        }
        // println!("{:?}", self.instructions);
    }

    fn do_cycle(&mut self) {
        self.draw_pixel();
        self.cycles += 1;

        if self.partial_instruction {
            self.partial_instruction = false;
            if let Instr::ADDX(cnt) = self.instructions.pop().unwrap() {
                self.register += cnt;
            } else {
                panic!()
            }
        } else {
            match self.instructions.pop().unwrap() {
                Instr::Noop => {}
                x => {
                    self.partial_instruction = true;
                    self.instructions.push(x);
                }
            }
        }
    }

    fn draw_pixel(&mut self) {
        if (self.cycles % SCREEN_WIDTH as i32) == self.register {
            self.screen[self.cycles as usize] = '#';
        } else if (self.cycles % SCREEN_WIDTH as i32) == self.register + 1 {
            self.screen[self.cycles as usize] = '#';
        } else if (self.cycles % SCREEN_WIDTH as i32) == self.register - 1 {
            self.screen[self.cycles as usize] = '#';
        }
    }
}

pub struct Problem10 {
    cpu: CPU,
}

impl Problem10 {
    pub fn new() -> Box<Self> {
        Box::new(Self { cpu: CPU::new() })
    }
}

impl Problem for Problem10 {
    fn part1(&mut self, input: &str) -> String {
        self.cpu = CPU::new();
        self.cpu.load_program(input);
        let mut sum = 0;

        while !self.cpu.instructions.is_empty() {
            self.cpu.do_cycle();
            let s = self.cpu.cycles - 20;
            if s >= 0 && s % 40 == 0 {
                // println!("cycle: {}", cpu.cycles);
                // println!("register: {}", cpu.register);
                // println!("sum: {}\n", cpu.cycles * cpu.register);
                sum += self.cpu.cycles * self.cpu.register;
            }
        }

        sum.to_string()
    }

    fn part2(&mut self, _input: &str) -> String {
        let a = self
            .cpu
            .screen
            .chunks(SCREEN_WIDTH)
            .map(|x| x.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");
        format!("\n{a}")
    }
}
