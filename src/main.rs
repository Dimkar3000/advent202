use colored::*;
use libadvent2022::*;
use std::{fs, time::Instant};

fn main() {
    let mut problems: Vec<Box<dyn Problem>> = vec![
        Problem1::new(),
        Problem2::new(),
        Problem3::new(),
        Problem4::new(),
        Problem5::new(),
        Problem6::new(),
        Problem7::new(),
        Problem8::new(),
        Problem9::new(),
        Problem10::new(),
    ];

    for (index, problem) in problems.iter_mut().enumerate() {
        let path = format!("inputs/input{}.txt", index + 1);
        let input = fs::read_to_string(&path).unwrap();

        let s = Instant::now();
        let r1 = problem.part1(&input);
        let t1 = s.elapsed();

        let s = Instant::now();
        let r2 = problem.part2(&input);
        let t2 = s.elapsed();

        let total = t1 + t2;

        println!(
            "{} {:02} (total time: {})",
            "Problem".bold(),
            (index + 1).to_string().bold().green(),
            total.as_secs_f64()
        );
        println!("==========");
        println!(
            "{} ({:.2}%): {}",
            "Result A".bold(),
            t1.as_secs_f64() / total.as_secs_f64(),
            r1.green()
        );
        println!(
            "{} ({:.2}%): {}\n",
            "Result B".bold(),
            t2.as_secs_f64() / total.as_secs_f64(),
            r2.green()
        );
    }
}
