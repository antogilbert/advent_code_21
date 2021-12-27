use std::{path::PathBuf, str::FromStr, collections::HashMap};

use crate::util;

use util::Runnable;

pub struct Day16 {
    file: String,
    hex_2_bin: HashMap<char, Vec<u8>>,
}

trait BasePkt {
    fn version(&self) -> u8;
    fn type_id(&self) -> u8;
}

trait LiteralValue {
    fn value(&self) -> u32;
}

trait Operator {
    fn length_type_id(&self) -> u8;
    fn length(&self) -> u32;
    fn subpackets(&self) -> Vec<char>;
}

struct Packet {
    data: Vec<char>,
}


impl Day16 {
    pub fn new(typ: &str) -> Day16 {
        let mut path = PathBuf::from_str(file!()).unwrap();
        path.pop();
        Day16 {
            file: String::from(path.to_str().unwrap()) + "/" + typ + ".txt",
            hex_2_bin: [
                ('0', vec![0, 0, 0, 0]),
                ('1', vec![0, 0, 0, 1]),
                ('2', vec![0, 0, 1, 0]),
                ('3', vec![0, 0, 1, 1]),
                ('4', vec![0, 1, 0, 0]),
                ('5', vec![0, 1, 0, 1]),
                ('6', vec![0, 1, 1, 0]),
                ('7', vec![0, 1, 1, 1]),
                ('8', vec![1, 0, 0, 0]),
                ('9', vec![1, 0, 0, 1]),
                ('A', vec![1, 0, 1, 0]),
                ('B', vec![1, 0, 1, 1]),
                ('C', vec![1, 1, 0, 0]),
                ('D', vec![1, 1, 0, 1]),
                ('E', vec![1, 1, 1, 0]),
                ('F', vec![1, 1, 1, 1]),
            ].into_iter().collect(),
        }
    }

    fn read(&self) -> String {
        if let Ok(mut lines) = util::read_lines(&self.file) {
            return lines.next().unwrap().unwrap();
        }
       "".to_owned() 
    }

    fn convert(&self, hexa: &str) -> Vec<u8> {
        let mut bits = Vec::new();
        hexa.chars().for_each(|c| bits.append(&mut self.hex_2_bin.get(&c).unwrap().clone()));
        bits
    }
}

impl Runnable for Day16 {
    fn run(&self) {
        let v = self.read();

        let binary = self.convert(&v);
        println!("BINARY: {:?}", binary);

        let mut i = 0;
        let mut vers_sum: u32 = 0;
        while i < binary.len() - 6 {
            let mut vers = binary[i] * 4;
            i += 1;
            vers += binary[i] * 2;
            i += 1;
            vers += binary[i];
            println!("VERSION: {}, i {}", vers, i);
            vers_sum += vers as u32;
            println!("VERSION SUM: {}", vers_sum);

            i += 1;
            let mut type_id = binary[i] * 4;
            i += 1;
            type_id += binary[i] * 2;
            i += 1;
            type_id += binary[i];
            i += 1;
            println!("TYPE ID: {}, i {}", type_id, i);

            if type_id == 4 {
                while binary[i] != 0 {
                    i += 5;
                }
                println!("i out of loop {}", i);
                i += 5;
                println!("i b4 pad {} mod {}", i, i%4);
            } else {
                if binary[i] == 0 {
                    // let mut bitcount: u32 = 0;
                    i += 1;
                    for _ in 0..15 {
                        // bitcount *= 2;
                        // bitcount += binary[i];
                        i += 1;
                    }
                } else {
                    i += 1;
                    for _ in 0..11 {
                        // bitcount *= 2;
                        // bitcount += binary[i];
                        i += 1;
                    }
                }
            }
        }

        println!(
            "Day16 Part 1 - Version sum: {:?}", vers_sum
        );
    }
}
