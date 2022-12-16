use rayon::prelude::*;
use std::cmp::Ordering;

use crate::Problem;

pub struct Problem15;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Interval {
    pub from: i64,
    pub to: i64,
}

impl Problem15 {
    pub fn new() -> Self {
        Self
    }
}
const YROW: i64 = 2000000;
fn calc_coverage(sensor: &(i64, i64), beacon: &(i64, i64)) -> ((i64, i64), i64) {
    let distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();

    (*sensor, distance)
}

fn proccess_row(regions: &mut [((i64, i64), i64)], row: i64) -> Option<(i64, i64)> {
    // sort regions to allow f
    regions.sort_by(|a, b| {
        let a_y_diff = (row - a.0 .1).abs();
        let b_y_diff = (row - b.0 .1).abs();
        let from_a = a.0 .0 - (a.1 - a_y_diff);
        let from_b = b.0 .0 - (b.1 - b_y_diff);

        if from_a < from_b {
            Ordering::Less
        } else if from_a == from_b {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });

    let mut start = 0;
    for ((x, y), dist) in regions.iter() {
        let y_diff = (row - y).abs();
        if y_diff > *dist {
            continue;
        }

        let from = x - (dist - y_diff);
        let to = *x + (*dist - y_diff);

        if from > 4000000 {
            break;
        }
        if from > start {
            return Some((from - 1, row));
            // start = from;
        }
        if to > start {
            start = to;
        }
    }
    None
}

impl Problem for Problem15 {
    fn part1(&mut self, input: &str) -> String {
        let mut regions: Vec<((i64, i64), i64)> = Vec::new();
        for line in input.lines() {
            let words: Vec<_> = line.split(|c| c == '=' || c == ',' || c == ':').collect();

            let sensor_x = words[1];
            let sensor_y = words[3];
            let beacon_x = words[5];
            let beacon_y = words[7];

            let s_x: i64 = sensor_x.parse().unwrap();
            let s_y: i64 = sensor_y.parse().unwrap();
            let div_x: i64 = beacon_x.parse().unwrap();
            let div_y: i64 = beacon_y.parse().unwrap();

            let sensor = (s_x, s_y);
            let beacon = (div_x, div_y);
            let coverage = calc_coverage(&sensor, &beacon);
            regions.push(coverage);
        }
        let mut start = i64::MIN;
        let mut count = 0;
        for ((x, y), dist) in regions.iter() {
            let y_diff = (YROW - y).abs();
            if y_diff > *dist {
                continue;
            }

            let p1 = *x - (*dist - y_diff);
            let p2 = *x + (*dist - y_diff);

            if p1 > start {
                start = p1;
            }
            if p2 > start {
                count += p2 - start;
            }
            if p2 > start {
                start = p2;
            }
        }

        count.to_string()
    }

    fn part2(&mut self, input: &str) -> String {
        // (center, range)
        let mut regions: Vec<((i64, i64), i64)> = Vec::new();
        for line in input.lines() {
            let words: Vec<_> = line.split(|c| c == '=' || c == ',' || c == ':').collect();

            let sensor_x = words[1];
            let sensor_y = words[3];
            let beacon_x = words[5];
            let beacon_y = words[7];

            let s_x: i64 = sensor_x.parse().unwrap();
            let s_y: i64 = sensor_y.parse().unwrap();
            let div_x: i64 = beacon_x.parse().unwrap();
            let div_y: i64 = beacon_y.parse().unwrap();

            let sensor = (s_x, s_y);
            let beacon = (div_x, div_y);
            let coverage = calc_coverage(&sensor, &beacon);
            regions.push(coverage);
        }
        regions.sort();

        let result = (0..4_000_000).find_map(|row| proccess_row(&mut regions, row));
        let result = result.unwrap();
        (result.0 * 4_000_000 + result.1).to_string()
    }
}
