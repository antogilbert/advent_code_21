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
        let v2 = v.clone();
        const DAYS_1: u32 = 80;

        let mut spawn = 0;
        for _ in 0..DAYS_1 {
            // print!("Iteration {}: {:?}", i, v);
            v.iter_mut()
                .for_each(|f| if *f > 0 { *f -= 1 } else { *f = 6 });

            let mut s = vec![8; spawn];
            v.append(&mut s);

            spawn = v.iter().filter(|&&i| i == 0).count();
        }

        println!("Day6 Part 1 - Lanternfish: {}", v.len());

        const DAYS_2: usize = 256;
        let mut total = 0;
        let mut fish_set: Vec<u64> = vec![0; DAYS_2 + 8 + 1];
        v2.iter().for_each(|f| fish_set[*f as usize] += 1);

        total += v2.len() as u64;

        for i in 0..DAYS_2 {
            let fish = fish_set[i];
            fish_set[i + 6 + 1] += fish;
            fish_set[i + 8 + 1] += fish;
            total += fish;
        }

        println!("Day6 Part 2 - Lanternfish: {}", total);
    }
}
