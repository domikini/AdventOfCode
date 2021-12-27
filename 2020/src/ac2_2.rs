#![allow(dead_code)]

use std::fs::File;
use regex::Regex;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

pub fn ac2_2() -> Result<(), Error>{
    let mut horizontal_pos:usize = 0;
    let mut depth:usize = 0;
    let mut aim:usize = 0;
    let v = read_a_file(File::open("input2")?)?;
    for line in v{
        let mut line = line.split(" ");
        let vec: Vec<&str> = line.collect();
        let direction = vec[0];
        let amount = vec[1];
        let amount_int:usize = amount.parse().unwrap();
        match direction {
            "forward" => {
                horizontal_pos += &amount_int;
                depth += &amount_int * &aim;
            },
            "down" => aim += &amount_int,
            "up" => aim -= &amount_int,
            _ => println!("{}", "no direction"),
        }
    }
    println!("Svar 2_2: {}", horizontal_pos * depth);
    Ok(())
}

fn read_a_file<R: Read>(io: R) -> Result<Vec<String>, Error>  {
    let br = BufReader::new(io);
    let mut v = vec![];
    for line in br.lines() {
        v.push(line?
            .trim()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?);
    }
    Ok(v)
}