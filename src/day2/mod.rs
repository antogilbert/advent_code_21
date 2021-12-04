use std::collections::HashMap;

use crate::util;

use util::Runnable;

const FWD: &str = "forward";
const UP: &str = "up";
const DWN: &str = "down";

pub struct Day2 {
    file: String,
}

impl  Day2 {
    pub fn new() -> Day2 {
        Day2 {
            file: String::from("./src/day2/input.txt"),
        }
    }

    fn read(&self) -> HashMap<String, i32> {
        let mut v = HashMap::new();
        if let Ok(lines) = util::read_lines(&self.file) {
            for line in lines {
                let l = line.unwrap();
                let mut t = l.split_whitespace();
                let k = t.next().unwrap();
                if let Some(value) = v.get_mut(k) {
                    *value += t.next().unwrap().parse::<i32>().unwrap();
                } else {
                    v.insert(k.to_owned(), t.next().unwrap().parse::<i32>().unwrap());
                } 
            }
        }
        v
    }
}

impl Runnable for Day2 {
    fn run(&self) {
        let v = self.read();

        let offset = v.get(DWN).unwrap() - v.get(UP).unwrap();
        let fwd = *v.get(FWD).unwrap();

        println!("Day2 Part 1 - H*D: {} * {} = {}", fwd, offset, offset * fwd);
    }
}