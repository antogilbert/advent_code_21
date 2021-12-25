use std::{collections::HashMap, path::PathBuf, str::FromStr};

use crate::util;

use util::Runnable;

const START: &str = "start";
const END: &str = "end";

pub struct Day12 {
    file: String,
}

impl Day12 {
    pub fn new(typ: &str) -> Day12 {
        let mut path = PathBuf::from_str(file!()).unwrap();
        path.pop();
        Day12 {
            file: String::from(path.to_str().unwrap()) + "/" + typ + ".txt",
        }
    }

    fn read(&self) -> HashMap<String, Vec<String>> {
        let mut v = HashMap::new();
        if let Ok(lines) = util::read_lines(&self.file) {
            for line in lines {
                let s = line.unwrap();
                let vs: Vec<String> = s.split('-').map(|x| x.to_owned()).collect();
                if vs[0] != END {
                    v.entry(vs[0].clone())
                        .and_modify(|s: &mut Vec<String>| s.push(vs[1].clone()))
                        .or_insert(vec![vs[1].clone()]);
                }
                if vs[0] != START && vs[1] != END {
                    v.entry(vs[1].clone())
                        .and_modify(|s| s.push(vs[0].clone()))
                        .or_insert(vec![vs[0].clone()]);
                }
            }
        }
        v
    }

    fn dfs(&self, node: &str, map: &HashMap<String, Vec<String>>, visited: &Vec<String>) -> Vec<String> {
        let mut paths = Vec::new();
        let mut new_vist = visited.clone();
        new_vist.push(String::from(node));

        if node == END {
            let path = new_vist.join(",");
            return vec![path];
        }

        let next = map.get(node).unwrap();
        for n in next {
            if n != START {
                if n.chars().nth(0).unwrap().is_uppercase() || !visited.contains(n) {
                    let temp_paths = self.dfs(n, map, &new_vist);
                    paths.extend(temp_paths);
                }
            }
        }
        paths
    }

    fn dfs2(&self, node: &str, map: &HashMap<String, Vec<String>>, visited: &Vec<String>, is_small_cave_visited_twice: bool) -> Vec<String> {
        let mut paths = Vec::new();
        let mut new_vist = visited.clone();
        new_vist.push(String::from(node));

        if node == END {
            let path = new_vist.join(",");
            return vec![path];
        }

        let next = map.get(node).unwrap();
        for n in next {
            if n != START {
                if n.chars().nth(0).unwrap().is_uppercase() || !visited.contains(n)  {
                    let temp_paths = self.dfs2(n, map, &new_vist, is_small_cave_visited_twice);
                    paths.extend(temp_paths);
                } else if !is_small_cave_visited_twice {
                    let temp_paths = self.dfs2(n, map, &new_vist, true);
                    paths.extend(temp_paths);
                }
            }
        }

        paths
    }
} 

impl Runnable for Day12 {
    fn run(&self) {
        let v = self.read();
        let paths = self.dfs(START, &v, &vec!["".to_owned()]);

        println!("Day12 Part 1 - Total paths: {:?}", paths.len());

        let new_paths = self.dfs2(START, &v, &vec!["".to_owned()], false);
        
        println!("Day12 Part 2 - Total paths: {:?}", new_paths.len());
    }
}
