use crate::util;

use util::Runnable;

pub struct Day3 {
    file: String,
    n_digits: usize,
}

impl  Day3 {
    pub fn new(typ: &str) -> Day3 {
        let nd = match typ {
            "sample" => 5,
            "input" => 12,
            _ => panic!("UNKNOWN TYPE")
        };

        Day3 {
            file: String::from("./src/day3/".to_owned() + typ + ".txt"),
            n_digits: nd,
        }
    }

    fn read(&self) -> Vec<u32> {
        let mut v = vec![0; self.n_digits + 1];
        let mut counter = 0;
        if let Ok(lines) = util::read_lines(&self.file) {
            for line in lines {
                for (i, c) in line.unwrap().chars().enumerate() {
                    v[i] += c.to_digit(10).unwrap();
                }
                counter += 1;
            }
        }
        v[self.n_digits] = counter;
        v
    }
}

impl Runnable for Day3 {
    fn run(&self) {
        let v = self.read();
        let mut gamma = 0;
        let mut eps = 0;
        let half = v[self.n_digits]/2;
        for i in 0..self.n_digits {
            gamma *= 2;
            eps *= 2;
            if v[i] > half {
                gamma += 1;
            } else {
                eps += 1;
            }
        }

        println!("Day3 Part 1 - consumption: v {:?} g {} e {} g*e {}", v, gamma, eps, gamma*eps);

    }
}