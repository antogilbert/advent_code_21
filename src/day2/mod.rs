use std::{collections::HashMap, ops::Index};

use crate::util;

use util::Runnable;

const FWD: &str = "forward";
const UP: &str = "up";
const DWN: &str = "down";

pub struct Day2 {
    file: String,
}

enum Direction {
    U,
    D,
    F
}
struct Instruction {
    pub dir: Direction,
    pub amt: i32,
}

impl Instruction {
    pub fn new(direction: &str, amt: i32) -> Instruction {
        match direction {
            "forward" => Instruction {dir: Direction::F, amt},
            "up" => Instruction {dir: Direction::U, amt},
            "down" => Instruction {dir: Direction::D, amt},
            _ => panic!("WRONG DIRECTION")
        }
    }
}

impl Day2 {
    pub fn new(typ: &str) -> Day2 {
        Day2 {
            file: String::from("./src/day2/".to_owned() + typ + ".txt"),
        }
    }

    fn read(&self) -> (HashMap<String, i32>, Vec<Instruction>) {
        let mut map = HashMap::new();
        let mut vec = Vec::new();
        if let Ok(lines) = util::read_lines(&self.file) {
            for line in lines {
                let l = line.unwrap();
                let mut t = l.split_whitespace();
                let k = t.next().unwrap();
                let v =t.next().unwrap().parse::<i32>().unwrap();
                if let Some(value) = map.get_mut(k) {
                    *value += v;
                } else {
                    map.insert(k.to_owned(), v);
                } 
                vec.push(Instruction::new(k, v));
            }
        }
       (map, vec) 
    }
}

impl Runnable for Day2 {
    fn run(&self) {
        let (hm, v) = self.read();

        let offset = hm.get(DWN).unwrap() - hm.get(UP).unwrap();
        let fwd = *hm.get(FWD).unwrap();

        println!("Day2 Part 1 - H*D: {} * {} = {}", fwd, offset, offset * fwd);

        let mut aim = 0;
        let mut depth = 0;
        let mut hor = 0;
        for ins in v {
            match ins.dir {
                Direction::D => {aim += ins.amt},
                Direction::U => {aim -= ins.amt},
                Direction::F => {hor += ins.amt; depth += aim * ins.amt},
            }
        }

        println!("Day2 Part 2 - H*D: {} * {} = {}", hor, depth, hor* depth);
    }
}