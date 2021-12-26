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
            for line in lines {
                let s = line.unwrap();
                let vs: Vec<&str> = s.split(" -> ").collect();
                v.insert(vs[0].to_owned(), vs[1].to_owned());
            }
        }
        (template, v)
    }

    fn expand_polymer(&self, base: &str, rules: &HashMap<String, String>) -> String {
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
        let t2 = t.clone();

        const STEPS_1: u32 = 10;

        for _ in 0..STEPS_1 {
            t = self.expand_polymer(&t, &rules);
        }

        let mut charmap: HashMap<String, u64> = HashMap::new();
        for c in t.as_bytes() {
            let s = String::from(*c as char);
            charmap.entry(s).and_modify(|e| *e += 1).or_insert(1);
        }

        let max = charmap.values().max().unwrap();
        let min = charmap.values().min().unwrap();

        println!("Day14 Part 1 - Difference: {}", max - min);

        charmap.clear();
        const STEPS_2: u32 = 40;

        let mut track_pairs = HashMap::new();
        let mut i = 1;
        charmap.entry(String::from(t2.as_bytes()[0] as char)).and_modify(|e| *e += 1).or_insert(1);
        while i < t2.len() {
            let c1 = t2.as_bytes()[i-1] as char;
            let c2 = t2.as_bytes()[i] as char;
            let mut pair = String::from(c1);
            pair.push(c2);
            track_pairs.entry(pair).and_modify(|e| *e += 1).or_insert(1);
            charmap.entry(String::from(c2)).and_modify(|e| *e += 1).or_insert(1);
            i += 1;
        }

        for _ in 0..STEPS_2 {
            let mut partial_map = HashMap::new();
            for (k, v) in track_pairs.iter_mut() {
                let new_char = rules.get(k).unwrap();

                let c1 = k.as_bytes()[0] as char;
                let c2 = k.as_bytes()[1] as char;

                let mut new_pair1 = String::from(c1);
                new_pair1.push_str(new_char);
                let mut new_pair2 = String::from(new_char);
                new_pair2.push(c2);

                partial_map.entry(new_pair1).and_modify(|e| *e += *v).or_insert(*v);
                partial_map.entry(new_pair2).and_modify(|e| *e += *v).or_insert(*v);
                charmap.entry(new_char.clone()).and_modify(|e| *e += *v).or_insert(*v);
            }
            track_pairs = partial_map;
        }

        let max = charmap.values().max().unwrap();
        let min = charmap.values().min().unwrap();

        println!("Day14 Part 2 - Difference: {} - {} = {}", max, min, max - min);
    }
}