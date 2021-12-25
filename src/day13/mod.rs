use std::{path::PathBuf, str::FromStr, collections::HashSet, borrow::BorrowMut};

use crate::util;

use util::Runnable;

pub struct Day13 {
    file: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Point { x, y }
    }

    fn fold(&mut self, f: &Fold) {
        match f.dir.as_str() {
            "y" => {self.fold_up(f.pos);},
            "x" => {self.fold_left(f.pos);},
            _ => panic!("STEREONZO, NUN C'E' STA DIREZIONE.")
        }
    }

    fn fold_up(&mut self, axis: u32) {
        if axis < self.y {
            let dist = self.y - axis;
            self.y = axis - dist;
        }
    }

    fn fold_left(&mut self, axis: u32) {
        if axis < self.x {
            let dist = self.x - axis;
            self.x = axis - dist;
        }
    }
}

#[derive(Debug)]
struct Fold {
    dir: String,
    pos: u32,
}

impl Fold {
    fn new(dir: String, pos: u32) -> Self {
        Fold { dir, pos }
    }
}

#[derive(Debug)]
struct Paper {
    grid: HashSet<Point>,
}

impl Paper {
    fn new() -> Self {
        Paper {grid: HashSet::new()}
    }

    fn add(&mut self, p: &Point) {
        self.grid.insert(p.clone());
    }

    /*
    fn fold(&mut self, f: &Fold) {
        match f.dir.as_str() {
            "y" => {self.fold_up(f.pos);},
            "x" => {self.fold_left(f.pos);},
            _ => panic!("STEREONZO, NUN C'E' STA DIREZIONE.")
        }
    }

    fn fold_up(&mut self, pos: u32) {
        self.grid.into_iter().for_each(|mut p| p.fold_up(pos));
    }

    fn fold_left(&mut self, pos: u32) {
        self.grid.into_iter().for_each(|mut p| p.fold_left(pos));
    }
    */
}

impl Day13 {
    pub fn new(typ: &str) -> Day13 {
        let mut path = PathBuf::from_str(file!()).unwrap();
        path.pop();
        Day13 {
            file: String::from(path.to_str().unwrap()) + "/" + typ + ".txt",
        }
    }

    fn read(&self) -> (Vec<Point>, Vec<Fold>) {
        let mut p = Vec::new();
        let mut f = Vec::new();
        let mut parse_folds = false;
        if let Ok(lines) = util::read_lines(&self.file) {
            for line in lines {
                let s = line.unwrap();
                if s.is_empty() {
                    parse_folds = true;
                    continue;
                }

                if parse_folds {
                    let tokens: Vec<String> = s.split_whitespace().map(|x| x.to_owned()).collect();
                    let fold: Vec<String> = tokens[2].split('=').map(|s| s.to_owned()).collect();
                    f.push(Fold::new(fold[0].clone(), fold[1].parse::<u32>().unwrap()));
                } else {
                    let ints: Vec<u32> = s.split(',').map(|x| x.parse::<u32>().unwrap()).collect();
                    p.push(Point::new(ints[0], ints[1]));
                }
            }
        }
        (p, f)
    }
}

impl Runnable for Day13 {
    fn run(&self) {
        let (mut p, f) = self.read();

        let mut paper =  HashSet::new();

        // f.iter().for_each(|fold| paper.fold(fold));

        let fold = f.first().unwrap();
        for pt in &mut p {
            // for fold in &f {
                pt.fold(fold);
            // }
        }

        p.iter().for_each(|pt| {paper.insert(pt.clone());});

        // println!("Day13 Part 1 - Total: {:?} \nfolds {:?}", p, f);
        println!("Day13 Part 1 - Total: {:?}", paper.len());
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold_up() {
        let mut p = Point::new(10, 10);
        p.fold_up(5);

        assert_eq!(p.x, 10);
        assert_eq!(p.y, 0);

        let mut p = Point::new(10, 10);
        p.fold_up(15);

        assert_eq!(p.x, 10);
        assert_eq!(p.y, 10);
    }

    #[test]
    fn test_fold_left() {
        let mut p = Point::new(10, 10);
        p.fold_left(5);

        assert_eq!(p.x, 0);
        assert_eq!(p.y, 10);

        let mut p = Point::new(10, 10);
        p.fold_left(15);

        assert_eq!(p.x, 10);
        assert_eq!(p.y, 10);
    }
}