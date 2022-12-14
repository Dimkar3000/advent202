use std::cmp::max;

use crate::Problem;

#[derive(Default, Debug)]

struct Tree(usize);

#[derive(Default, Debug)]
pub struct Problem8 {
    grid: Vec<Vec<Tree>>,
}

impl Problem8 {
    pub fn new() -> Self {
        Self::default()
    }

    fn proccess_input(&mut self, input: &str) {
        for line in input.lines() {
            let new_row = line
                .chars()
                .map(|x| Tree(x.to_digit(10).unwrap() as usize))
                .collect();
            self.grid.push(new_row);
        }
    }

    fn visibility(&self, x: usize, y: usize) -> usize {
        let height = self.grid[x][y].0;

        let visible_left = (0..x).map(|i| self.grid[i][y].0).fold(0, |current, new| {
            if new >= height {
                return 1;
            }
            current + 1
        });

        let visible_right = ((x + 1)..self.grid[0].len())
            .rev()
            .map(|i| self.grid[i][y].0)
            .fold(0, |current, new| {
                if new >= height {
                    return 1;
                }

                current + 1
            });

        let visible_top = (0..y).map(|i| self.grid[x][i].0).fold(0, |current, new| {
            if new >= height {
                return 1;
            }

            current + 1
        });

        let visible_down = ((y + 1)..self.grid.len())
            .rev()
            .map(|i| self.grid[x][i].0)
            .fold(0, |current, new| {
                if new >= height {
                    return 1;
                }

                current + 1
            });

        visible_down * visible_top * visible_left * visible_right
    }

    fn visible(&self, x: usize, y: usize) -> bool {
        let height = self.grid[x][y].0;

        let mut visible_left = true;
        for i in 0..x {
            if self.grid[i][y].0 >= height {
                visible_left = false;
                break;
            }
        }
        if visible_left {
            return true;
        }

        let mut visible_right = true;
        for i in (x + 1)..self.grid[0].len() {
            if self.grid[i][y].0 >= height {
                visible_right = false;
                break;
            }
        }
        if visible_right {
            return true;
        }

        let mut visible_top = true;
        for i in 0..y {
            if self.grid[x][i].0 >= height {
                visible_top = false;
                break;
            }
        }
        if visible_top {
            return true;
        }

        let mut visible_down = true;
        for i in (y + 1)..self.grid.len() {
            if self.grid[x][i].0 >= height {
                visible_down = false;
                break;
            }
        }
        if visible_down {
            return true;
        }

        false
    }
}

impl Problem for Problem8 {
    fn part1(&mut self, input: &str) -> String {
        self.proccess_input(input);
        let mut count = 0;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                if self.visible(x, y) {
                    count += 1;
                }
            }
        }
        count.to_string()
    }

    fn part2(&mut self, _input: &str) -> String {
        let mut visibility = 0;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                visibility = max(visibility, self.visibility(x, y));
            }
        }
        visibility.to_string()
    }
}
