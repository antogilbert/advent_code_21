use crate::util;

use util::Runnable;

pub struct Day5 {
    file: String,
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Vent {
    start: Point,
    end: Point
}

impl Vent {
    pub fn new(start: Point, end: Point) -> Self {
        if start.x == end.x {
            if end.y < start.y {
                return Vent {
                    start: Point::new(start.x, end.y),
                    end: Point::new(end.x, start.y)
                }
            }
        }
        if start.y == end.y {
            if end.x < start.x {
                return Vent {
                    start: Point::new(end.x, start.y),
                    end: Point::new(start.x, end.y)
                }
            }
        }
        Vent { start, end }
    }
}

impl  Day5 {
    pub fn new(typ: &str) -> Day5 {
        Day5 {
            file: String::from("./src/day5/".to_owned() + typ + ".txt"),
        }
    }

    fn read(&self) -> Vec<Vent> {
        let mut v = Vec::new();
        if let Ok(lines) = util::read_lines(&self.file) {
            for line in lines {
                let s = line.unwrap();
                let mut sw = s.split_whitespace();
                let p1 = sw.next().unwrap().split(',').map(|c| c.parse::<u32>().unwrap()).collect::<Vec<u32>>();
                sw.next();
                let p2 = sw.next().unwrap().split(',').map(|c| c.parse::<u32>().unwrap()).collect::<Vec<u32>>();

                v.push(Vent::new(Point::new(p1[0], p1[1]), Point::new(p2[0], p2[1])));
            }
        }
        v
    }
}

impl Runnable for Day5 {
    fn run(&self) {
        let vs = self.read();

        let mut max = 0;

        for v in &vs {
            max = std::cmp::max(max, std::cmp::max(v.start.x, v.end.x));
            max = std::cmp::max(max, std::cmp::max(v.start.y, v.end.y));
        }

        max += 1;

        let mut grid = vec![vec![0; max as usize]; max as usize];


        let mut overlap = 0;
        for v in vs {
            if v.start.x == v.end.x {
                for j in v.start.y..=v.end.y {
                    grid[j as usize][v.start.x as usize] += 1;
                }
            }
            
            if v.start.y == v.end.y {
                for i in v.start.x..=v.end.x {
                    grid[v.start.y as usize][i as usize] += 1;
                }
            }
        }

        for row in grid {
            for i in row {
                if i > 1 {
                    overlap += 1;
                }
            }
        }

        println!("Number of overlaps {}", overlap);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}