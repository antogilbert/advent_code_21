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

    fn basin_size(&self, sink: (i32, i32), grid: &mut Vec<Vec<u32>>) -> u32 {
        if sink.0 < 0
            || sink.1 < 0
            || sink.0 >= grid.len() as i32
            || sink.1 >= grid[0].len() as i32
            || grid[sink.0 as usize][sink.1 as usize] == 9
        {
            return 0;
        }

        grid[sink.0 as usize][sink.1 as usize] = 9;
        let l = (sink.0, sink.1 - 1);
        let r = (sink.0, sink.1 + 1);
        let u = (sink.0 - 1, sink.1);
        let d = (sink.0 + 1, sink.1);

        self.basin_size(l, grid)
            + self.basin_size(r, grid)
            + self.basin_size(u, grid)
            + self.basin_size(d, grid)
            + 1
    }
}

impl Runnable for Day9 {
    fn run(&self) {
        let v = self.read();
        let mut lowpts: Vec<u32> = Vec::new();
        let mut basins: Vec<(i32, i32)> = Vec::new();

        // inner matrix
        for (i, row) in v.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                let it = *item;
                let uc = i as i32;
                let lc = j as i32;

                let l = if (lc - 1) < 0 { u32::MAX } else { v[i][j - 1] };
                let r = if (j + 1) >= v[0].len() {
                    u32::MAX
                } else {
                    v[i][j + 1]
                };
                let u = if (uc - 1) < 0 { u32::MAX } else { v[i - 1][j] };
                let d = if (i + 1) >= v.len() {
                    u32::MAX
                } else {
                    v[i + 1][j]
                };

                if it < l && it < r && it < u && it < d {
                    lowpts.push(it);
                    basins.push((i as i32, j as i32));
                }
            }
        }

        let total: u32 = lowpts.iter().sum::<u32>() + lowpts.len() as u32;

        println!("Day9 Part 1 - Total: {}", total);

        let mut basin_mult = 0;
        let mut sizes = Vec::new();
        if !basins.is_empty() {
            basin_mult = 1;
            for b in basins {
                let bsize = self.basin_size(b, &mut v.clone());
                sizes.push(bsize);
            }
        }

        sizes.sort_by(|a, b| b.cmp(a));

        for i in 0..3 {
            basin_mult *= sizes[i];
        }
        println!("Day9 Part 2 - Total: {}", basin_mult);
    }
}
