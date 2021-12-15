use std::{collections::HashMap, path::PathBuf, str::FromStr};

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

    fn read(&self) -> Vec<(String, String)> {
        let mut v = Vec::new();
        if let Ok(lines) = util::read_lines(&self.file) {
            for line in lines {
                let s = line.unwrap();
                let mut split = s.split('|');

                v.push((
                    split.next().unwrap().to_owned(),
                    split.next().unwrap().to_owned(),
                ));
            }
        }
        v
    }

    fn sort_signal(&self, signal: &mut String) {
        let mut s: Vec<char> = signal.chars().collect();
        s.sort_unstable();
        *signal = s.into_iter().collect();
    }
}

impl Runnable for Day8 {
    fn run(&self) {
        let v = self.read();

        let mut total = 0;

        for (signals, digits) in v {
            let mut vs: Vec<String> = signals.split_whitespace().map(|s| s.to_owned()).collect();
            let mut vd: Vec<String> = digits.split_whitespace().map(|s| s.to_owned()).collect();

            vs.iter_mut().for_each(|s| self.sort_signal(s));
            vd.iter_mut().for_each(|s| self.sort_signal(s));

            let mut sixes = Vec::new();
            let mut fives = Vec::new();

            let mut one = String::new();
            let mut four = String::new();
            let mut seven = String::new();

            let mut sig_map = HashMap::new();

            for signal in vs {
                match signal.len() {
                    2 => {
                        one = signal.clone();
                        sig_map.insert(signal, 1);
                    }
                    3 => {
                        seven = signal.clone();
                        sig_map.insert(signal, 7);
                    }
                    4 => {
                        four = signal.clone();
                        sig_map.insert(signal, 4);
                    }
                    5 => {
                        fives.push(signal);
                    }
                    6 => {
                        sixes.push(signal);
                    }
                    7 => {
                        sig_map.insert(signal, 8);
                    }
                    _ => {}
                }
            }

            let mut six = String::new();
            for signal in sixes {
                let mut counter = 0;
                for c in signal.chars() {
                    if four.contains(c) || seven.contains(c) {
                        counter += 1;
                    }
                }

                if counter == 5 {
                    sig_map.insert(signal, 9);
                } else if counter == 4 {
                    for c in one.chars() {
                        if signal.contains(c) {
                            counter += 1;
                        }
                    }

                    if counter == 6 {
                        sig_map.insert(signal, 0);
                    } else {
                        six = signal.clone();
                        sig_map.insert(signal, 6);
                    }
                }
            }

            let mut five = String::new();
            for signal in &fives {
                let mut counter = 0;
                for c in signal.chars() {
                    if six.contains(c) {
                        counter += 1;
                    }
                }

                if counter == 5 {
                    five = signal.clone();
                    sig_map.insert(signal.to_string(), 5);
                    break;
                }
            }

            for signal in fives {
                let mut counter = 0;
                for c in signal.chars() {
                    if five.contains(c) {
                        counter += 1;
                    }
                }

                if counter == 4 {
                    sig_map.insert(signal, 3);
                } else if counter == 3 {
                    sig_map.insert(signal, 2);
                }
            }

            let mut number = 0;

            for signal in vd {
                number *= 10;
                number += sig_map.get(&signal).unwrap();
            }

            total += number;
        }
        println!("Day8 Part 2 - Total: {}", total);
    }
}
