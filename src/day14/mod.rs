use std::{path::PathBuf, str::FromStr, collections::HashMap};

use crate::util;

use util::Runnable;

pub struct Day14 {
    file: String,
}

impl Day14 {
    pub fn new(typ: &str) -> Day14 {
        let mut path = PathBuf::from_str(file!()).unwrap();
        path.pop();
        Day14 {
            file: String::from(path.to_str().unwrap()) + "/" + typ + ".txt",
        }
    }

    fn read(&self) -> (String, HashMap<String, String>) {
        let mut v = HashMap::new();
        let mut template = String::new();
        if let Ok(mut lines) = util::read_lines(&self.file) {
            template = lines.next().unwrap().unwrap(); 
            lines.next();
            while let Some(line) = lines.next() {
                let s = line.unwrap();
                let vs: Vec<&str> = s.split("->").collect();
                v.insert(String::from(vs[0].trim().to_owned()), String::from(vs[1].trim().to_owned()));
            }
        }
        (template, v)
    }

    fn expand_polymer(&self, base: &String, rules: &HashMap<String, String>) -> String {
        let mut polymer = String::new();
        let mut i = 1;
        polymer.push(base.as_bytes()[0] as char);
        while i < base.len() {
            let b_base = base.as_bytes();
            let c1 = b_base[i-1] as char;
            let c2 = b_base[i] as char;
            let mut key = String::from(c1);
            
            key.push(c2);
            let cm = rules.get(&key).unwrap();

            polymer.push_str(cm);
            polymer.push(c2);

            i += 1 ;
        }
        polymer
    }
}

impl Runnable for Day14 {
    fn run(&self) {
        let (mut t, rules) = self.read();

        const STEPS: u32 = 10;

        for _ in 0..STEPS {
            t = self.expand_polymer(&t, &rules);
        }

        let mut charmap = HashMap::new();

        for c in t.as_bytes() {
            charmap.entry(*c as char).and_modify(|e| *e += 1).or_insert(1);
        }

        let max = charmap.values().max().unwrap();
        let min = charmap.values().min().unwrap();

        println!("Day14 Part 1 - Difference: {}", max - min);
    }
}