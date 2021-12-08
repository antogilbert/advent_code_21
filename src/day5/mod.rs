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
    end: Point,
    diag: bool
}

impl Vent {
    pub fn new(start: Point, end: Point) -> Self {
        Vent {
            diag: (start.x != end.x) && (start.y != end.y),
            start, 
            end 
        }
    }

    pub fn update_grid(&self, grid: &mut Vec<Vec<u32>>) {
        let mut range_x = self.start.x..self.end.x;
        let mut range_y = self.start.y..self.end.y;

        if !self.diag {
            if self.start.x > self.end.x {
                range_x = self.end.x..self.start.x;
            }
            if self.start.y > self.end.y {
                range_y = self.end.y..self.start.y;
            }
            if range_x.is_empty() {
                for j in range_y.clone() {
                    grid[j as usize][self.start.x as usize] += 1;
                }
                grid[range_y.end as usize][self.start.x as usize] += 1;
            } else if range_y.is_empty(){
                for i in range_x.clone() {
                    grid[self.start.y as usize][i as usize] += 1;
                }
                grid[self.start.y as usize][range_x.end as usize] += 1;
            }
        } else {
            let mut i = range_x.start;
            let mut j = range_y.start;

            while i != range_x.end {
                while j != range_y.end {
                    grid[j as usize][i as usize] += 1;

                    if self.start.y > self.end.y {
                        j -= 1;
                    } else {
                        j += 1;
                    }
                    
                    break;
                }
                if self.start.x > self.end.x {
                    i -= 1;
                } else {
                    i += 1;
                }
            }
            grid[j as usize][i as usize] += 1;
        }
    }
}

impl  Day5 {
    pub fn new(typ: &str) -> Day5 {
        Day5 {
            file: "./src/day5/".to_owned() + typ + ".txt",
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


        println!("{:?}", vs);
        let mut overlap = 0;
        for v in vs {
            v.update_grid(&mut grid);
        }

        for row in &grid {
            for i in row {
                if *i > 1 {
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

    #[test]
    fn test_perpendicular_grid_update() {
        let vents = vec![
            Vent::new(Point::new(0,9), Point::new(5,9)),
            Vent::new(Point::new(9,4), Point::new(3,4)),
            Vent::new(Point::new(2,2), Point::new(2,1)),
            Vent::new(Point::new(7,0), Point::new(7,4)),
            Vent::new(Point::new(0,9), Point::new(2,9)),
            Vent::new(Point::new(3,4), Point::new(1,4))];

        let mut grid = vec![vec![0; 10]; 10];

        for v in vents {
            v.update_grid(&mut grid);
        }

        let expected_grid= vec![
            vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 1, 1, 2, 1, 1, 1, 2, 1, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![2, 2, 2, 1, 1, 1, 0, 0, 0, 0],
        ];

        assert_eq!(expected_grid, grid);
    }

    #[test]
    fn test_full_grid_update() {
        let vents = vec![
            Vent::new(Point::new(0,9), Point::new(5,9)),
            Vent::new(Point::new(8,0), Point::new(0,8)),
            Vent::new(Point::new(9,4), Point::new(3,4)),
            Vent::new(Point::new(2,2), Point::new(2,1)),
            Vent::new(Point::new(7,0), Point::new(7,4)),
            Vent::new(Point::new(6,4), Point::new(2,0)),
            Vent::new(Point::new(0,9), Point::new(2,9)),
            Vent::new(Point::new(3,4), Point::new(1,4)),
            Vent::new(Point::new(0,0), Point::new(8,8)),
            Vent::new(Point::new(5,5), Point::new(8,2)),
        ];

        let mut grid = vec![vec![0; 10]; 10];

        for v in vents {
            v.update_grid(&mut grid);
        }

        let expected_grid = vec![
            vec![1, 0, 1, 0, 0, 0, 0, 1, 1, 0],
            vec![0, 1, 1, 1, 0, 0, 0, 2, 0, 0],
            vec![0, 0, 2, 0, 1, 0, 1, 1, 1, 0],
            vec![0, 0, 0, 1, 0, 2, 0, 2, 0, 0],
            vec![0, 1, 1, 2, 3, 1, 3, 2, 1, 1],
            vec![0, 0, 0, 1, 0, 2, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 1, 0],
            vec![2, 2, 2, 1, 1, 1, 0, 0, 0, 0],
        ];

        assert_eq!(expected_grid, grid);
    }
}