use std::{path::PathBuf, str::FromStr};

use crate::util;

use util::Runnable;

pub struct Day8 {
    file: String,
}

impl Day8 {
    pub fn new(typ: &str) -> Day8 {
        let mut path = PathBuf::from_str(file!()).unwrap();
        path.pop();
        Day8 {
            file: String::from(path.to_str().unwrap()) + "/" + typ + ".txt",
        }
    }

    fn read(&self) -> Vec<String> {
        let mut v = Vec::new();
        if let Ok(lines) = util::read_lines(&self.file) {
            for line in lines {
                let s = line.unwrap();
                let k: Vec<String> = s
                    .split('|')
                    .map(|f| f.to_owned())
                    .collect();

                let mut k2: Vec<String> = k[1]
                    .split_whitespace()
                    .filter_map(|f| if f.len() == 2 || f.len() == 3 || f.len() == 4 || f.len() == 7 {Some(f.to_owned())} else {None} )
                    .collect();

                v.append(&mut k2);
            }
        }
        v
    }
}

impl Runnable for Day8 {
    fn run(&self) {
        let v = self.read();

        println!("Day8 Part 1 - Easy digits appearance: {:?}", v.len());
    }
}
