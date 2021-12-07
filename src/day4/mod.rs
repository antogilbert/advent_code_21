use crate::util;

use util::Runnable;

pub struct Day4 {
    file: String,
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Board {
    board: Vec<Vec<i32>>,
    pub is_bingo: bool,
    pub win_turn: u32,
    pub win_val: i32
}

impl PartialOrd for Board {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.win_turn.cmp(&other.win_turn))
    }
}

impl Board {
    fn new() -> Self {
        Board {
            board: Vec::new(),
            is_bingo: false,
            win_turn: u32::MAX,
            win_val: 0
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

    pub fn check_bingo(&mut self) {
        let mut bingo = true;
        for row in &self.board {
            bingo = true;
            for i in row {
                bingo = bingo && (*i < 0);
            }
            if bingo {
                self.is_bingo = bingo;
                return;
            }
        }

        for i in 0..self.board.len() {
            bingo = true;
            for j in 0..self.board.len() {
                bingo = bingo && (self.board[j][i] < 0)
            }
            if bingo {
                self.is_bingo = bingo;
                return;
            }
        }
        self.is_bingo = false;
    }

    pub fn score(&self) -> i32 {
        let mut sum = 0;
        for row in &self.board {
            sum += row.into_iter().filter(|x| **x > 0).fold(0, |acc, x| acc + x);
        }
        println!("SUM {}", sum);
        sum*self.win_val
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
        for (c, i) in v.into_iter().enumerate() {
            winning = i as i32;
            for b in bs.iter_mut() {
                if b.is_bingo {
                    b.win_turn = std::cmp::min(b.win_turn, c as u32);
                    continue;
                }
                b.mark(i as i32);
                b.check_bingo();
                b.win_val = winning;
            }
        }

        bs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for b in bs.into_iter().rev() {
            if b.is_bingo {
                println!("Final Score: {}, winning {}, order {}", b.score(), b.win_val, b.win_turn);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_bingo() {
        let mut b = Board::new();

        for _ in 0..4 {
            b.add_row(vec![1; 5]);
        }
        b.add_row(vec![-1; 5]);
        b.check_bingo();

        assert!(b.is_bingo);
    }

    #[test]
    fn test_col_bingo() {
        let mut b = Board::new();

        let row = vec![-1, 1, 1, 1, 1];
        for _ in 0..5 {
            b.add_row(row.clone());
        }
        b.check_bingo();

        assert!(b.is_bingo);
    }

    #[test]
    fn test_mark() {
        let mut b = Board::new();

        let row = vec![1, 2, 3, 4, 5];
        for i in 1..6 {
            b.add_row((&row).into_iter().map(|x| i*x).collect());
        }

        b.mark(1);

        assert_eq!(b.board[0][0], -1);
    }
}