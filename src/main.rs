use advent_code_21::*;
use util::Runnable;

fn main() {
    let day = std::env::args().nth(1).expect("NO DAY PROVIDED");
    let typ = std::env::args().nth(2).expect("NO INPUT PROVIDED");

    let runners: Vec<Box<dyn Runnable>> = vec![
        Box::new(day1::Day1::new(&typ)),
        Box::new(day2::Day2::new(&typ)),
        Box::new(day3::Day3::new(&typ)),
        Box::new(day4::Day4::new(&typ)),
        Box::new(day5::Day5::new(&typ)),
        Box::new(day6::Day6::new(&typ)),
        Box::new(day7::Day7::new(&typ)),
        Box::new(day8::Day8::new(&typ)),
        Box::new(day9::Day9::new(&typ)),
        Box::new(day10::Day10::new(&typ)),
        Box::new(day11::Day11::new(&typ)),
        Box::new(day12::Day12::new(&typ)),
        Box::new(day13::Day13::new(&typ)),
        Box::new(day14::Day14::new(&typ)),
    ];

    match day.parse::<usize>() {
        Ok(d) => {
            runners[d - 1].run();
        }
        Err(_) => {
            runners.iter().for_each(|r| r.run());
        }
    }
}
