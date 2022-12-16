use std::collections::HashSet;

use crate::Problem;

pub struct Problem14;

impl Problem14 {
    pub fn new() -> Self {
        Self
    }
}

fn simulate_sand(from: &(i64, i64), walls: &[(i64, i64)], sands: &[(i64, i64)]) -> (i64, i64) {
    let mut current = *from;
    loop {
        current.1 += 1;
        if current.1 > walls.last().unwrap().1 {
            return (i64::MIN, i64::MIN);
        }

        let contains = |x: &(i64, i64)| walls.contains(x) || sands.contains(x);

        let beneath = (current.0, current.1 + 1);
        let beneath_left = (current.0 - 1, current.1 + 1);
        let beneath_right = (current.0 + 1, current.1 + 1);

        if contains(&beneath) && contains(&beneath_left) && contains(&beneath_right) {
            return current;
        } else if contains(&beneath) && !contains(&beneath_left) {
            current.0 -= 1;
        } else if contains(&beneath) && !contains(&beneath_right) {
            current.0 += 1;
        }
    }
}

impl Problem for Problem14 {
    fn part1(&mut self, input: &str) -> String {
        let mut points: HashSet<(i64, i64)> = HashSet::new();
        for line in input.lines() {
            let line_points: Vec<(i64, i64)> = line
                .split(" -> ")
                .map(|x| {
                    let (x, y) = x.split_once(',').unwrap();
                    (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
                })
                .collect();
            for w in line_points.windows(2) {
                let f = w[0];
                let s = w[1];

                // horizontal line
                if f.0 == s.0 {
                    if f.1 > s.1 {
                        points.extend(((s.1)..=(f.1)).map(|y| (f.0, y)));
                    } else {
                        points.extend(((f.1)..=(s.1)).map(|y| (f.0, y)));
                    }
                }

                // vertical line
                if f.1 == s.1 {
                    if f.0 > s.0 {
                        points.extend(((s.0)..=(f.0)).map(|x| (x, f.1)));
                    } else {
                        points.extend(((f.0)..=(s.0)).map(|x| (x, f.1)));
                    }
                }
            }
        }
        let mut walls: Vec<(i64, i64)> = points.iter().cloned().collect();
        walls.sort_by(|a, b| (a.1).partial_cmp(&b.1).unwrap());

        let mut sands: Vec<(i64, i64)> = Vec::new();
        let mut count = 0;
        loop {
            let point = simulate_sand(&(500, 0), &walls, &sands);
            if point.1 == i64::MIN || sands.contains(&point) {
                break;
            }
            count += 1;
            sands.push(point);
        }
        sands.sort_by(|a, b| (a.1).partial_cmp(&b.1).unwrap());
        count.to_string()
    }

    fn part2(&mut self, input: &str) -> String {
        let mut points: HashSet<(i64, i64)> = HashSet::new();
        for line in input.lines() {
            let line_points: Vec<(i64, i64)> = line
                .split(" -> ")
                .map(|x| {
                    let (x, y) = x.split_once(',').unwrap();
                    (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
                })
                .collect();
            for w in line_points.windows(2) {
                let f = w[0];
                let s = w[1];

                // horizontal line
                if f.0 == s.0 {
                    if f.1 > s.1 {
                        points.extend(((s.1)..=(f.1)).map(|y| (f.0, y)));
                    } else {
                        points.extend(((f.1)..=(s.1)).map(|y| (f.0, y)));
                    }
                }

                // vertical line
                if f.1 == s.1 {
                    if f.0 > s.0 {
                        points.extend(((s.0)..=(f.0)).map(|x| (x, f.1)));
                    } else {
                        points.extend(((f.0)..=(s.0)).map(|x| (x, f.1)));
                    }
                }
            }
        }
        let mut walls: Vec<(i64, i64)> = points.iter().cloned().collect();
        walls.sort_by(|a, b| (a.1).partial_cmp(&b.1).unwrap());

        let floor = walls.last().unwrap().1 + 2;
        println!("floor: {floor}");
        for i in -10000..1000 {
            walls.push((i, floor));
        }

        let mut sands: Vec<(i64, i64)> = Vec::new();
        let mut count = 0;
        loop {
            let point = simulate_sand(&(500, -1), &walls, &sands);
            // println!("sand at: {:?}", point);
            if point.0 == 500 && point.1 == 0 {
                count += 1;
                break;
            }
            count += 1;
            sands.push(point);
        }
        sands.sort_by(|a, b| (a.1).partial_cmp(&b.1).unwrap());
        // println!("{:?}", sands);
        count.to_string()
    }
}
