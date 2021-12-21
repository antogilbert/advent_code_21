use std::{path::PathBuf, str::FromStr, collections::HashMap};

use crate::util;

use util::Runnable;

pub struct Day10 {
    file: String,
}

impl Day10 {
    pub fn new(typ: &str) -> Day10 {
        let mut path = PathBuf::from_str(file!()).unwrap();
        path.pop();
        Day10 {
            file: String::from(path.to_str().unwrap()) + "/" + typ + ".txt",
        }
    }

    fn read(&self) -> Vec<String> {
        let mut v = Vec::new();
        if let Ok(lines) = util::read_lines(&self.file) {
            for line in lines {
                let s = line.unwrap();
                v.push(s);
            }
        }
        v
    }
}

impl Runnable for Day10 {
    fn run(&self) {
        let v = self.read();
        let closed: HashMap<char, u32> = [
            (')', 3),
            (']', 57),
            ('}', 1197),
            ('>', 25137),
        ].into_iter().collect();

        let open: HashMap<char, char> = [
            (')', '('),
            (']', '['),
            ('}', '{'),
            ('>', '<'),
        ].into_iter().collect();

        let mut total = 0;
        for s in v {
            let mut stack = Vec::new();

            for c in s.chars() {
                if !closed.contains_key(&c) {
                    stack.push(c);
                } else {
                    let o = stack.pop().unwrap();
                    if *open.get(&c).unwrap() != o {
                        total += closed.get(&c).unwrap();
                    }
                }
            }
        }

        println!("Day10 Part 1 - Total: {}", total);
    }
}
