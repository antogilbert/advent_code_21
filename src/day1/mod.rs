use crate::util;

use util::Runnable;

pub struct Day1 {
    file: String,
}

impl  Day1 {
    pub fn new() -> Day1 {
        Day1 {
            file: String::from("./src/day1/input.txt"),
        }
    }

    fn read(&self) -> Vec<i32> {
        let mut v = Vec::new();
        if let Ok(lines) = util::read_lines(&self.file) {
            v = lines.map(|line| line.unwrap().parse::<i32>().unwrap()).collect();
        }
        v
    }
}

impl Runnable for Day1 {
    fn run(&self) {
        let mut increments = 0;
        let mut previous = i32::MAX;
        for i in self.read() {
            if i > previous {
                increments += 1;
            }
            previous = i;
        }

        println!("Day1 - Increments: {}", increments);
    }
}