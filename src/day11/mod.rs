use std::{path::PathBuf, str::FromStr};

use crate::util;

use util::Runnable;

pub struct Day11 {
    file: String,
}

#[derive(Debug, Clone)]
struct Octopus {
    value: u32,
    flashed: bool,
}

impl Octopus {
    fn default(value: u32) -> Self {
        Octopus {
            value,
            flashed: false,
        }
    }

    fn new(value: u32, flashed: bool) -> Self {
        Octopus { value, flashed }
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    fn reset(&mut self) {
        self.value = 0;
        self.flashed = false;
    }

    fn needs_neighbours_processing(&mut self) -> bool {
        if !self.flashed {
            self.increment();
            if self.value > 9 {
                self.flashed = true;
                return true;
            }
        }
        false
    }
}

impl Day11 {
    pub fn new(typ: &str) -> Day11 {
        let mut path = PathBuf::from_str(file!()).unwrap();
        path.pop();
        Day11 {
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

    fn get_grid(&self, ss: &[String]) -> Vec<Vec<Octopus>> {
        let mut v = Vec::new();
        let size = ss[0].len() + 2;
        v.push(vec![Octopus::new(9, true); size]);
        for s in ss {
            let mut row = vec![Octopus::new(9, true)];
            for c in s.chars() {
                row.push(Octopus::default(c.to_digit(10).unwrap()));
            }
            row.push(Octopus::new(9, true));
            v.push(row);
        }
        v.push(vec![Octopus::new(9, true); size]);
        v
    }

    fn increment_energy(&self, grid: &mut Vec<Vec<Octopus>>) {
        for i in 1..(grid.len() - 1) {
            for j in 1..(grid[0].len() - 1) {
                let x = &mut grid[i][j];
                x.increment();
                if x.value > 9 && !x.flashed {
                    x.flashed = true;
                    self.process_neighbours((i, j), grid);
                }
            }
        }
    }

    fn process_neighbours(&self, coord: (usize, usize), grid: &mut Vec<Vec<Octopus>>) {
        if coord.1 > 0 {
            let left = (coord.0, coord.1 - 1);
            let x = &mut grid[left.0][left.1];
            if x.needs_neighbours_processing() {
                self.process_neighbours(left, grid);
            }

            if coord.0 > 0 {
                let u_left = (coord.0 - 1, left.1);
                let x = &mut grid[u_left.0][u_left.1];
                if x.needs_neighbours_processing() {
                    self.process_neighbours(u_left, grid);
                }
            }

            if coord.0 < grid.len() - 1 {
                let d_left = (coord.0 + 1, left.1);
                let x = &mut grid[d_left.0][d_left.1];
                if x.needs_neighbours_processing() {
                    self.process_neighbours(d_left, grid);
                }
            }
        }

        if coord.0 < grid[0].len() - 1 {
            let right = (coord.0, coord.1 + 1);
            let x = &mut grid[right.0][right.1];
            if x.needs_neighbours_processing() {
                self.process_neighbours(right, grid);
            }

            if coord.0 > 0 {
                let u_right = (coord.0 - 1, right.1);
                let x = &mut grid[u_right.0][u_right.1];
                if x.needs_neighbours_processing() {
                    self.process_neighbours(u_right, grid);
                }
            }

            if coord.0 < grid.len() - 1 {
                let d_right = (coord.0 + 1, right.1);
                let x = &mut grid[d_right.0][d_right.1];
                if x.needs_neighbours_processing() {
                    self.process_neighbours(d_right, grid);
                }
            }
        }

        if coord.0 > 0 {
            let up = (coord.0 - 1, coord.1);
            let x = &mut grid[up.0][up.1];
            if x.needs_neighbours_processing() {
                self.process_neighbours(up, grid);
            }
        }

        if coord.0 < grid.len() - 1 {
            let down = (coord.0 + 1, coord.1);
            let x = &mut grid[down.0][down.1];
            if x.needs_neighbours_processing() {
                self.process_neighbours(down, grid);
            }
        }
    }

    fn count_flashes(&self, grid: &mut Vec<Vec<Octopus>>) -> i32 {
        let mut flashes = 0;
        for i in 1..(grid.len() - 1) {
            for j in 1..(grid[0].len() - 1) {
                let x = &mut grid[i][j];
                if x.value > 9 {
                    flashes += 1;
                    x.reset();
                }
            }
        }
        flashes
    }
}

impl Runnable for Day11 {
    fn run(&self) {
        let v = self.read();
        let mut flashes = 0;
        let mut grid = self.get_grid(&v);
        let mut grid2 = grid.clone();

        const STEPS: u32 = 100;

        for _ in 0..STEPS {
            self.increment_energy(&mut grid);
            let step_flash = self.count_flashes(&mut grid);
            flashes += step_flash;
        }

        println!("Day11 Part 1 - Total flashes: {}", flashes);

        let mut step = (0, false);
        while !step.1 {
            self.increment_energy(&mut grid2);
            let step_flash = self.count_flashes(&mut grid2);
            step.0 += 1;
            if step_flash == 100 {
                step.1 = true;
            }
        }
        println!("Day11 Part 2 - Full flash step: {}", step.0);
    }
}
