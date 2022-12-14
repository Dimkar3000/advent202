use std::{
    cmp::{max, min},
    collections::HashSet,
};

use crate::Problem;

#[derive(Default)]
pub struct Problem9;

#[derive(Debug, Clone, Copy)]
enum Direction {
    U,
    D,
    R,
    L,
}

#[derive(Debug, Clone, Copy)]
struct Movement(Direction, usize);

impl Movement {
    fn from_line(input: &str) -> Self {
        let (f, s) = input.split_once(' ').unwrap();

        let count = s.parse::<usize>().unwrap();

        match f {
            "U" => Movement(Direction::U, count),
            "D" => Movement(Direction::D, count),
            "L" => Movement(Direction::L, count),
            "R" => Movement(Direction::R, count),
            _ => unreachable!(),
        }
    }

    fn adjust(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
        let mut x_dif = head.0 - tail.0;
        let mut y_dif = head.1 - tail.1;

        if x_dif.abs() < 2 && y_dif.abs() < 2 {
            return tail;
        }
        x_dif = max(min(x_dif, 1), -1);
        y_dif = max(min(y_dif, 1), -1);

        (tail.0 + x_dif, tail.1 + y_dif)
    }

    fn do_move(direction: Direction, head: (i32, i32)) -> (i32, i32) {
        let mut new_head = head;
        match direction {
            Direction::U => {
                new_head.1 += 1;
            }
            Direction::D => {
                new_head.1 -= 1;
            }
            Direction::L => {
                new_head.0 -= 1;
            }
            Direction::R => {
                new_head.0 += 1;
            }
        };
        new_head
    }
}

impl Problem9 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Problem for Problem9 {
    fn part1(&mut self, input: &str) -> String {
        let movements: Vec<Movement> = input.lines().map(Movement::from_line).collect();

        let mut visited = HashSet::new();
        let mut head_pos = (0, 0);
        let mut tail_pos = (0, 0);
        visited.insert(tail_pos);
        movements.iter().for_each(|movement| {
            for _ in 0..movement.1 {
                head_pos = Movement::do_move(movement.0, head_pos);
                tail_pos = Movement::adjust(head_pos, tail_pos);
                visited.insert(tail_pos);
            }
        });
        visited.len().to_string()
    }

    fn part2(&mut self, input: &str) -> String {
        let movements: Vec<Movement> = input.lines().map(Movement::from_line).collect();

        let mut visited = HashSet::new();
        let mut knots = vec![(0, 0); 10];
        visited.insert(knots[0]);
        movements.iter().for_each(|movement| {
            for _ in 0..movement.1 {
                let head_pos = Movement::do_move(movement.0, knots[0]);
                knots[0] = head_pos;

                for i in 0..(knots.len() - 1) {
                    let n2 = Movement::adjust(knots[i], knots[i + 1]);
                    knots[i + 1] = n2;
                    // println!("{:?}, {:?}", knots[i], knots[i + 1])
                }
                visited.insert(*knots.last().unwrap());
            }
        });
        visited.len().to_string()
    }
}
#[cfg(test)]
mod tests {
    use crate::{Problem, Problem9};

    use super::Movement;

    #[test]
    fn simple_up() {
        let head = (1, 2);
        let tail = (0, 0);
        let n = Movement::adjust(head, tail);
        assert_eq!(n.0, 1);
        assert_eq!(n.1, 1);
    }

    #[test]
    fn wierd_up() {
        let head = (1, 1);
        let tail = (-1, 0);
        let n = Movement::adjust(head, tail);
        assert_eq!(n.0, 0);
        assert_eq!(n.1, 1);
    }

    #[test]
    fn dont_move() {
        let head = (0, 1);
        let tail = (0, 0);
        let n = Movement::adjust(head, tail);
        assert_eq!(n.0, tail.0);
        assert_eq!(n.1, tail.1);
    }
}
