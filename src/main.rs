mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod util;

use util::Runnable;

fn main() {
    let day = std::env::args().nth(1).expect("NO DAY PROVIDED");
    let typ = std::env::args().nth(2).expect("NO DAY PROVIDED");
    println!("Running day #{}", day);


    let runners: Vec<Box<dyn Runnable>> = vec![
        Box::new(day1::Day1::new(&typ)),
        Box::new(day2::Day2::new(&typ)),
        Box::new(day3::Day3::new(&typ)),
        Box::new(day4::Day4::new(&typ)),
        Box::new(day5::Day5::new(&typ)),
        Box::new(day6::Day6::new(&typ)),
    ];

    runners[day.parse::<usize>().unwrap() - 1].run();
}
