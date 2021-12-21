use std::{path::PathBuf, str::FromStr};

use crate::util;

use util::Runnable;

pub struct Day9 {
    file: String,
}

impl Day9 {
    pub fn new(typ: &str) -> Day9 {
        let mut path = PathBuf::from_str(file!()).unwrap();
        path.pop();
        Day9 {
            file: String::from(path.to_str().unwrap()) + "/" + typ + ".txt",
        }
    }

    fn read(&self) -> Vec<Vec<u32>> {
        let mut v = Vec::new();
        if let Ok(lines) = util::read_lines(&self.file) {
            for line in lines {
                let s = line.unwrap();
                let mut row = Vec::new();
                for c in s.chars() {
                    row.push(c.to_digit(10).unwrap());
                }
                v.push(row);
            }
        }
        v
    }
}

impl Runnable for Day9 {
    fn run(&self) {
        let v = self.read();
        let mut lowpts: Vec<u32> = Vec::new();

        // inner matrix
        for (i, row) in v.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                let it = *item;
                if it == 0 {
                    lowpts.push(it);
                } else {
                    let uc = i as i32;
                    let lc = j as i32;

                    let l = if (lc - 1) < 0 {
                        u32::MAX
                    } else {
                        v[i][j-1]
                    };
                    let r = if (j + 1) >= v[0].len() {
                        u32::MAX
                    } else {
                        v[i][j+1]
                    };
                    let u = if (uc - 1) < 0 {
                        u32::MAX
                    } else {
                        v[i-1][j]
                    };
                    let d = if (i + 1) >= v.len() {
                        u32::MAX
                    } else {
                        v[i+1][j]
                    };

                    if it < l && it < r && it < u && it < d {
                        lowpts.push(it);
                    }
                }
            }
        }

        let total: u32 = lowpts.iter().sum::<u32>() + lowpts.len() as u32;

        println!("Day9 Part 1 - Total: {}", total);
    }
}
