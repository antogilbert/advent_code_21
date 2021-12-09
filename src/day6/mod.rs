use std::{path::PathBuf, str::FromStr};

use crate::util;

use util::Runnable;

pub struct Day6 {
    file: String,
}

impl Day6 {
    pub fn new(typ: &str) -> Day6 {
        let mut path = PathBuf::from_str(file!()).unwrap();
        path.pop();
        Day6 {
            file: String::from(path.to_str().unwrap()) + "/" + typ + ".txt",
        }
    }

    fn read(&self) -> Vec<i32> {
        let mut v = Vec::new();
        if let Ok(lines) = util::read_lines(&self.file) {
            for line in lines {
                let s = line.unwrap();
                let mut k = s
                    .split(',')
                    .map(|c| c.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                v.append(&mut k);
            }
        }
        v
    }
}

impl Runnable for Day6 {
    fn run(&self) {
        let mut v = self.read();
        const DAYS: u32 = 80;

        let mut spawn = 0;
        for _ in 0..DAYS {
            // print!("Iteration {}: {:?}", i, v);
            v.iter_mut()
                .for_each(|f| if *f > 0 { *f -= 1 } else { *f = 6 });

            let mut s = vec![8; spawn];
            v.append(&mut s);

            spawn = v.iter().filter(|&&i| i == 0).count();
        }

        println!("Day6 Part 1 - Lanternfish: {}", v.len());
    }
}
