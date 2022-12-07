use colored::*;
use libadvent2022::*;
use std::fs;

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
    ];

    for (index, problem) in problems.iter_mut().enumerate() {
        let path = format!("inputs/input{}.txt", index + 1);
        let input = fs::read_to_string(&path).unwrap();

        let r1 = problem.part1(&input);
        let r2 = problem.part2(&input);

        println!(
            "{} {:02}",
            "Problem".bold(),
            (index + 1).to_string().bold().green()
        );
        println!("==========");
        println!("{}: {}", "Result A".bold(), r1.green());
        println!("{}: {}\n", "Result B".bold(), r2.green());
    }
}
