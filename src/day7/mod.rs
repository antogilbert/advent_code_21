use std::{path::{Path, PathBuf}, str::FromStr};

use crate::util;

use util::Runnable;

pub struct Day7 {
    file: String,
}

impl  Day7 {
    pub fn new(typ: &str) -> Day7 {
        let mut path = PathBuf::from_str(file!()).unwrap();
        path.pop();
        Day7 {
            file: String::from(path.to_str().unwrap()) + "/" + typ + ".txt",
        }
    }

    fn read(&self) -> Vec<i32> {
        let mut v = Vec::new();
        if let Ok(lines) = util::read_lines(&self.file) {
            for line in lines {
                let s = line.unwrap();
                let mut k = s.split(',').map(|c| c.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                v.append(&mut k);
            }
        }
        v
    }
}

impl Runnable for Day7 {
    fn run(&self) {
        let v = self.read();
        let max = *v.iter().max().unwrap();

        let mut pos = vec![0; (max + 1) as usize];
        let mut pos2 = vec![0; (max + 1) as usize];

        for (i, c) in pos.iter_mut().enumerate() {
            for x in &v {
                let m = (x - (i as i32)).abs();
                *c += m;
                pos2[i] += (m+1)*m/2
            }
        }

        println!("Day7 Part 1 - Crab fuel: {}", pos.iter().min().unwrap());
        println!("Day7 Part 2 - Crab fuel: {}", pos2.iter().min().unwrap());
    }
}