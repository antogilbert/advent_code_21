use crate::util;

use util::Runnable;

pub struct Day4 {
    file: String,
}

#[derive(Debug)]
struct Board {
    board: Vec<Vec<i32>>,
    pub is_bingo: bool
}

impl Board {
    fn new() -> Self {
        Board {
            board: Vec::new(),
            is_bingo: false
        }
    }    
    
    pub fn add_row(&mut self, row: Vec<i32>) {
        assert!(self.board.len() < 5);
        self.board.push(row);
    }

    pub fn mark(&mut self, num: i32) {
        for row in self.board.iter_mut() {
            for i in row {
                if *i == num {
                    *i = -(*i);
                    if *i == 0 { *i = -1 }
                    return;
                }
            }
        }
    }

    pub fn check_bingo(&mut self) -> bool {
        let mut bingo = true;
        for row in &self.board {
            bingo = true;
            for i in row {
                bingo = bingo && (*i < 0);
            }
            if bingo {
                self.is_bingo = bingo;
                return bingo;
            }
        }

        for i in 0..self.board.len() {
            bingo = true;
            for j in 0..self.board.len() {
                bingo = bingo && (self.board[i][j] < 0)
            }
            if bingo {
                self.is_bingo = bingo;
                return bingo;
            }
        }
        self.is_bingo = false;
        false 
    }

    pub fn score(&self, count: i32) -> i32 {
        let mut sum = 0;
        for row in &self.board {
            sum += row.into_iter().filter(|x| **x > 0).fold(0, |acc, x| acc + x);
        }
        sum*count
    }
}


impl  Day4 {
    pub fn new(typ: &str) -> Day4 {
        Day4 {
            file: String::from("./src/day4/".to_owned() + typ + ".txt"),
        }
    }

    fn read(&self) -> (Vec<u32>, Vec<Board>) {
        let mut v = Vec::new();
        let mut b = Vec::new();
        if let Ok(lines) = util::read_lines(&self.file) {
            let mut board_num = -2;
            for line in lines {
                let s = line.unwrap();
                if board_num == -2 {
                    v = s.split(',').map(|x| x.parse::<u32>().unwrap()).collect();
                    board_num += 1;
                    continue;
                }

                if s.is_empty() {
                    b.push(Board::new());
                    board_num += 1;
                } else {
                    let row = s.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
                    b[board_num as usize].add_row(row);
                }
            }
        }
        (v, b)
    }
}

impl Runnable for Day4 {
    fn run(&self) {
        let (v, mut bs) = self.read();

        let mut winning= 0;
        let mut bingo = false;
        println!("V: {:?}", v);
        for i in v {
            winning = i as i32;
            for b in bs.iter_mut() {
                b.mark(i as i32);
                b.check_bingo();
                if b.is_bingo {
                    bingo = true;
                    break;
                }
            }

            if bingo {break;}
        }

        for b in bs {
            if b.is_bingo {
                println!("Final Score: {}", b.score(winning));
            }
        }
    }
}