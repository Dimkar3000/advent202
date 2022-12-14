use crate::Problem;
use pathfinding::prelude::bfs;

pub struct Problem12 {
    elevations: Vec<Vec<i32>>,
    start: (i32, i32),
    end: (i32, i32),
}

impl Problem12 {
    pub fn new() -> Self {
        Self {
            elevations: Vec::new(),
            start: (0, 0),
            end: (0, 0),
        }
    }

    fn skeleton(
        &self,
        start: (i32, i32),
        callback: &dyn Fn((i32, i32), (i32, i32)) -> bool,
        end: &dyn Fn((i32, i32)) -> bool,
    ) -> i32 {
        let result = bfs(
            &start,
            |(x, y)| {
                let mut poses = vec![(*x - 1, *y), (*x, *y - 1), (*x + 1, *y), (*x, *y + 1)];
                poses = poses
                    .iter()
                    .filter(|(x0, y0)| {
                        if *x0 < 0 {
                            return false;
                        }
                        if *y0 < 0 {
                            return false;
                        }
                        if *x0 >= (self.elevations.len() as i32) {
                            return false;
                        }
                        if *y0 >= (self.elevations[0].len() as i32) {
                            return false;
                        }
                        true
                    })
                    .filter(|&&(x0, y0)| callback((x0, y0), (*x, *y)))
                    .copied()
                    .collect();
                poses.sort();
                poses
            },
            |(x, y)| end((*x, *y)),
        );
        (result.unwrap().len() - 1) as i32
    }
}

impl Problem for Problem12 {
    fn part1(&mut self, input: &str) -> String {
        for (i, line) in input.lines().enumerate() {
            self.elevations.push(Vec::new());
            for (j, ch) in line.chars().enumerate() {
                if ch == 'S' {
                    self.start = (i as i32, j as i32);
                    self.elevations[i].push(0);
                } else if ch == 'E' {
                    self.end = (i as i32, j as i32);
                    self.elevations[i].push((b'z' - b'a') as i32);
                } else {
                    self.elevations[i].push(((ch as u8) - b'a') as i32);
                }
            }
        }

        let branchs = |(x, y), (x0, y0)| {
            if self.elevations[x as usize][y as usize] - self.elevations[x0 as usize][y0 as usize]
                >= 2
            {
                return false;
            }
            true
        };
        let r = self.skeleton(self.start, &branchs, &|(x, y)| {
            self.end.0 == x && self.end.1 == y
        });

        r.to_string()
    }

    fn part2(&mut self, _input: &str) -> String {
        let branches = |(x, y), (x0, y0)| {
            if self.elevations[x0 as usize][y0 as usize] - self.elevations[x as usize][y as usize]
                >= 2
            {
                return false;
            }
            true
        };
        let r = self.skeleton(self.end, &branches, &|(x, y)| {
            self.elevations[x as usize][y as usize] == 0
        });

        r.to_string()
    }
}
