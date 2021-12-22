use std::{collections::HashMap, path::PathBuf, str::FromStr};

use crate::util;

use util::Runnable;

pub struct Day10 {
    file: String,
}

#[derive(Debug)]
struct Score {
    syntax: u32,
    autocomplete: u32,
}

impl Day10 {
    pub fn new(typ: &str) -> Day10 {
        let mut path = PathBuf::from_str(file!()).unwrap();
        path.pop();
        Day10 {
            file: String::from(path.to_str().unwrap()) + "/" + typ + ".txt",
        }
    }

    fn read(&self) -> Vec<String> {
        let mut v = Vec::new();
        if let Ok(lines) = util::read_lines(&self.file) {
            for line in lines {
                let s = line.unwrap();
                v.push(s);
            }
        }
        v
    }
}

impl Runnable for Day10 {
    fn run(&self) {
        let v = self.read();
        let score: HashMap<char, Score> = [
            (
                ')',
                Score {
                    syntax: 3,
                    autocomplete: 1,
                },
            ),
            (
                ']',
                Score {
                    syntax: 57,
                    autocomplete: 2,
                },
            ),
            (
                '}',
                Score {
                    syntax: 1197,
                    autocomplete: 3,
                },
            ),
            (
                '>',
                Score {
                    syntax: 25137,
                    autocomplete: 4,
                },
            ),
        ]
        .into_iter()
        .collect();

        let open: HashMap<char, char> = [(')', '('), (']', '['), ('}', '{'), ('>', '<')]
            .into_iter()
            .collect();

        let closed: HashMap<char, char> = open.iter().map(|k| (*k.1, *k.0)).collect();

        let mut syntax = 0;
        let mut autocomplete = Vec::new();

        for s in v {
            let mut stack = Vec::new();

            for c in s.chars() {
                if !open.contains_key(&c) {
                    stack.push(c);
                } else if let Some(o) = stack.pop() {
                    if let Some(par) = open.get(&c) {
                        if *par != o {
                            syntax += score.get(&c).unwrap().syntax;
                            stack.clear();
                            break;
                        }
                    }
                }
            }

            let mut stackscore: u64 = 0;
            while let Some(c) = stack.pop() {
                if let Some(sc) = closed.get(&c) {
                    stackscore *= 5;
                    stackscore += score.get(sc).unwrap().autocomplete as u64;
                }
            }

            if stackscore > 0 {
                autocomplete.push(stackscore);
            }
        }

        autocomplete.sort();

        println!("Day10 Part 1 - Total: {}", syntax);
        let i: usize = (autocomplete.len()-1)/2;
        println!("Day10 Part 2 - Total: {:?}", autocomplete[i]);
    }
}
