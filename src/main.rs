mod day1;
mod day2;
// mod day3;
mod util;

use util::Runnable;

fn main() {
    let day = std::env::args().nth(1).expect("NO DAY PROVIDED");
    let typ = std::env::args().nth(2).expect("NO DAY PROVIDED");
    println!("Running day #{}", day);

    let d1: Box<dyn Runnable> = Box::new(day1::Day1::new(&typ));
    let d2: Box<dyn Runnable> = Box::new(day2::Day2::new(&typ));
    // let d3: Box<dyn Runnable> = Box::new(day3::Day3::new());

    let mut runners: Vec<Box<dyn Runnable>> = Vec::new();
    runners.push(d1);
    runners.push(d2);
    // runners.push(d3);

    runners[day.parse::<usize>().unwrap() - 1].run();
}
