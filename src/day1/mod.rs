use crate::util;

use util::Runnable;

pub struct Day1 {
    file: String,
}

impl  Day1 {
    pub fn new(typ: &str) -> Day1 {
        Day1 {
            file: "./src/day1/".to_owned() + typ + ".txt",
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
        let v = self.read();
        for i in &v {
            if i > &previous {
                increments += 1;
            }
            previous = *i;
        }

        println!("Day1 Part 1- Increments: {}", increments);

        let mut sliding_inc = 0;
        let mut sliding_prev = v[0] + v[1] + v[2];

        for i in 3..v.len() {
            let new_val = sliding_prev + v[i] - v[i-3];
            if new_val > sliding_prev {
                sliding_inc += 1;
            }
            sliding_prev = new_val;
        }
        println!("Day1 Part 2 - Window Increments: {}", sliding_inc);
    }
}