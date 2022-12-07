use crate::Problem;

#[derive(Default, Debug)]
pub struct Problem6 {}

impl Problem6 {
    pub fn new() -> Box<Self> {
        Box::new(Self::default())
    }

    pub fn skeleton(input: &str, size: usize) -> String {
        let mut result = "no result!".to_string();
        let mut current = String::new();
        input.chars().enumerate().any(|(index, next)| {
            current.push(next);
            if current.len() > size {
                current.remove(0);
            }

            if current.len() < size {
                return false;
            }

            if has_no_duplicate_letters(&current) {
                result = (index + 1).to_string();
                return true;
            }
            false
        });
        result
    }
}

fn has_no_duplicate_letters(s: &str) -> bool {
    let mut found = [false; 26]; // all lower case letters in the input
    for c in s.chars() {
        let index = ((c as u8) - b'a') as usize;
        if found[index] {
            return false;
        }
        found[index] = true;
    }
    true
}

impl Problem for Problem6 {
    fn part1(&mut self, input: &str) -> String {
        Problem6::skeleton(input, 4)
    }

    fn part2(&mut self, input: &str) -> String {
        Problem6::skeleton(input, 14)
    }
}
