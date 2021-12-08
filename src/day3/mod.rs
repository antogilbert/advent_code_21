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
            file: "./src/day3/".to_owned() + typ + ".txt",
            n_digits: nd,
        }
    }

    fn read(&self) -> (Vec<u32>, Vec<String>) {
        let mut v = vec![0; self.n_digits + 1];
        let mut vs = Vec::new();
        let mut counter = 0;
        if let Ok(lines) = util::read_lines(&self.file) {
            for line in lines {
                let x = line.unwrap();
                vs.push(x.clone());
                for (i, c) in x.chars().enumerate() {
                    v[i] += c.to_digit(10).unwrap();
                }
                counter += 1;
            }
        }
        v[self.n_digits] = counter;
        (v, vs)
    }

    fn get_ratings(&self, lines: &[String], is_most: bool) -> String {
        let mut ans = lines.to_owned();

        for i in 0..self.n_digits {
            let mut counter = 0;
            for line in &ans {
                counter += line.chars().nth(i).unwrap().to_digit(10).unwrap();
            }

            let len = ans.len() as u32;
            let x = if counter*2 >= len {1} else {0};

            if is_most {
                ans = ans.into_iter().filter(|s| s.chars().nth(i).unwrap().to_digit(10).unwrap() == x).collect();
            } else {
                ans = ans.into_iter().filter(|s| s.chars().nth(i).unwrap().to_digit(10).unwrap() != x).collect();
            }

            if ans.len() == 1 {
                return ans[0].clone();
            }
        }

        ans[0].clone()
    }
}

impl Runnable for Day3 {
    fn run(&self) {
        let (v, vs) = self.read();
        let mut gamma = 0;
        let mut eps = 0;
        let half = v[self.n_digits]/2;
        for i in v.iter().take(self.n_digits) {
            gamma *= 2;
            eps *= 2;
            if *i > half {
                gamma += 1;
            } else {
                eps += 1;
            }
        }

        println!("Day3 Part 1 - consumption: v {:?} g {} e {} g*e {}", v, gamma, eps, gamma*eps);

        let oxy_str = self.get_ratings(&vs, true);
        let co2_str = self.get_ratings(&vs, false);

        let mut oxy = 0;
        let mut co2 = 0;

        for i in 0..oxy_str.len() {
            oxy *= 2;
            co2 *= 2;

            oxy += oxy_str.chars().nth(i).unwrap().to_digit(10).unwrap();
            co2 += co2_str.chars().nth(i).unwrap().to_digit(10).unwrap();
        }

        println!("Day3 Part 2 - Oxygen {} CO2 {}, LSR {}", oxy, co2, oxy*co2);
    }
}