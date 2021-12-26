use std::{
    collections::{BinaryHeap, HashMap},
    path::PathBuf,
    str::FromStr,
};

use crate::util;

use util::Runnable;

pub struct Day15 {
    file: String,
}

impl Day15 {
    pub fn new(typ: &str) -> Day15 {
        let mut path = PathBuf::from_str(file!()).unwrap();
        path.pop();
        Day15 {
            file: String::from(path.to_str().unwrap()) + "/" + typ + ".txt",
        }
    }

    fn read(&self) -> Vec<Vec<u32>> {
        let mut v = Vec::new();
        if let Ok(lines) = util::read_lines(&self.file) {
            for line in lines {
                let s = line.unwrap();
                v.push(s.chars().map(|c| c.to_digit(10).unwrap()).collect());
            }
        }
        v
    }

    fn neighbours(&self, node: &(usize, usize), grid: &[Vec<u32>]) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();

        // up 
        if node.0 > 0 {
            neighbours.push((node.0 - 1, node.1));
        }
        // left
        if node.1 > 0 {
            neighbours.push((node.0, node.1 - 1));
        }
        // down 
        if node.0 < grid[0].len() - 1 {
            neighbours.push((node.0 + 1, node.1));
        }
        // right 
        if node.1 < grid.len() - 1 {
            neighbours.push((node.0, node.1 + 1));
        }

        neighbours
    }

    fn dijkstra(&self, grid: &[Vec<u32>]) -> u32 {
        let max_r = grid.len();
        let max_c = grid[0].len();
        let goal = (max_r - 1, max_c - 1);

        let mut provenance = HashMap::new();

        let mut frontier = BinaryHeap::new();
        frontier.push(Node::new((0, 0), 0));

        let mut risk= HashMap::new();
        risk.entry((0, 0)).or_insert(0);

        while let Some(node) = frontier.pop() {
            if node.coords == goal {
                break;
            }

            for next in self.neighbours(&node.coords, grid) {
                let new_risk = *risk.get(&node.coords).unwrap() + grid[next.0][next.1];

                if !risk.contains_key(&next) || new_risk < *risk.get(&next).unwrap() {
                    risk.insert(next, new_risk);
                    frontier.push(Node::new(next, new_risk as usize));
                    provenance.insert(next, node.clone());
                }
            }
        }
        *risk.get(&goal).unwrap()
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Node {
    coords: (usize, usize),
    priority: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.priority.partial_cmp(&self.priority)
    }
}

impl Node {
    fn new(coords: (usize, usize), priority: usize) -> Self {
        Node { coords, priority }
    }
}

impl Runnable for Day15 {
    fn run(&self) {
        let grid = self.read();

        println!(
            "Day15 Part 1 - Total risk: {:?}", self.dijkstra(&grid)
        );

        let max_r = grid.len();
        let max_c = grid[0].len();
        let mut full_grid = vec![vec![0; 5 * max_c]; 5 * max_r];

        // First tile row:
        for k in 0..5_usize {
            for i in 0..max_r {
                for j in 0..max_c {
                    let mut v = grid[i][j] as usize + k;
                    if v > 9 {
                        v -= 9;
                    }
                    full_grid[i][max_c * k + j] = v as u32;
                }
            }
        }

        for k in 1..5_usize {
            for i in 0..max_r {
                for j in 0..full_grid[0].len() {
                    let mut v = full_grid[i][j] as usize + k;
                    if v > 9 {
                        v -= 9;
                    }

                    full_grid[max_r * k + i][j] = v as u32;
                }
            }
        }

        println!(
            "Day15 Part 2 - Total risk: {:?}", self.dijkstra(&full_grid)
        );
    }
}
