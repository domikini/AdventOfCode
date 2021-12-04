#![allow(dead_code)]

use std::fs::File;
use regex::Regex;
use regex::RegexSet;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

pub fn ac4_1() -> Result<(), Error>{
    let v = read_a_file(File::open("input4")?)?;
    let mut bingo_sequence = &v[0].split(",").collect::<Vec<_>>();
    let mut bingo_sequence_int:Vec<usize> = bingo_sequence.into_iter().map(|number|number.parse().unwrap()).collect();
    println!("{:?}", bingo_sequence_int);
    for line in v {
        // println!("{}", line);
    }
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