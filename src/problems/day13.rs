use std::cmp::Ordering;

use crate::Problem;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    Integer(i32),
    List(Vec<Item>),
}

impl Item {
    fn push(&mut self, item: Item) {
        match self {
            Item::List(l) => l.push(item),
            Item::Integer(_) => unreachable!(),
        }
    }

    fn new(input: &str) -> Self {
        let mut stack: Vec<Item> = Vec::new();
        let mut current = Item::List(Vec::new());
        let mut num_agreagator = String::new();

        for c in input.chars() {
            match c {
                '[' => {
                    stack.push(current.clone());
                    current = Item::List(Vec::new());
                }
                ']' => {
                    if !num_agreagator.is_empty() {
                        current.push(Item::Integer(num_agreagator.parse().unwrap()));
                        num_agreagator.clear();
                    }

                    stack.last_mut().unwrap().push(current.clone());

                    current = stack.pop().unwrap();
                }
                ',' => {
                    if !num_agreagator.is_empty() {
                        current.push(Item::Integer(num_agreagator.parse().unwrap()));
                        num_agreagator.clear();
                    }
                }
                a => {
                    if a.is_digit(10) {
                        num_agreagator.push(a);
                    } else {
                        unreachable!()
                    }
                }
            }
        }
        match current {
            Item::List(x) => x[0].clone(),
            _ => unreachable!(),
        }
        // Item::internal(0, input).0
    }
}

pub struct Problem13;

impl Problem13 {
    pub fn new() -> Self {
        Self
    }

    fn right_order(left: &Item, right: &Item) -> i32 {
        match (left, right) {
            (Item::Integer(x), Item::Integer(y)) => x - y,
            (Item::List(left_list), Item::Integer(y)) => {
                let l = Item::List(left_list.clone());
                let r = Item::List(vec![Item::Integer(*y)]);
                Problem13::right_order(&l, &r)
            }
            (Item::Integer(x), Item::List(right_list)) => {
                let r = Item::List(right_list.clone());
                let l = Item::List(vec![Item::Integer(*x)]);
                Problem13::right_order(&l, &r)
            }

            (Item::List(left_list), Item::List(right_list)) => {
                let mut index = 0;
                loop {
                    let l = left_list.get(index);
                    let r = right_list.get(index);
                    if l.is_some() && r.is_some() {
                        let result = Problem13::right_order(l.unwrap(), r.unwrap());
                        if result == 0 {
                            index += 1;
                            continue;
                        }
                        return result;
                    }
                    if l.is_none() && r.is_some() {
                        return -1;
                    }
                    if r.is_none() && l.is_some() {
                        return 1;
                    }

                    return 0;
                }
            }
        }
    }
}

impl Problem for Problem13 {
    fn part1(&mut self, input: &str) -> String {
        let items: Vec<_> = input
            .lines()
            .filter(|x| !x.is_empty())
            .map(Item::new)
            .collect();
        let results: usize = items
            .chunks(2)
            .map(|x| Problem13::right_order(&x[0], &x[1]))
            .map(|x| x.is_negative())
            .enumerate()
            .filter(|x| x.1)
            .map(|x| x.0 + 1)
            .sum();
        results.to_string()
    }

    fn part2(&mut self, input: &str) -> String {
        let mut items: Vec<_> = input
            .lines()
            .filter(|x| !x.is_empty())
            .map(Item::new)
            .collect();

        let div1 = Item::List(vec![Item::List(vec![Item::Integer(2)])]);
        let div2 = Item::List(vec![Item::List(vec![Item::Integer(6)])]);
        items.push(div1.clone());
        items.push(div2.clone());

        items.sort_by(|a, b| {
            let cond = Problem13::right_order(a, b);

            if cond < 0 {
                Ordering::Less
            } else if cond == 0 {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        });

        let mut div1index = 0;
        let mut div2index = 0;
        for (i, x) in items.iter().enumerate() {
            if *x == div1 {
                div1index = i + 1
            }
            if *x == div2 {
                div2index = i + 1
            }
        }

        (div1index * div2index).to_string()
    }
}
