use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::io::Result;
use std::path::Path;

pub trait Runnable {
    fn run(&self);
}

pub fn read_lines<P>(file: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let f = File::open(file)?;
    Ok(BufReader::new(f).lines())
}
