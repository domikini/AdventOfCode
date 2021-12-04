#![allow(dead_code)]

use std::fs::File;
use regex::Regex;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};


pub fn ac3_1() -> Result<(), Error>{
    let mut gamma: String = "".to_string();
    let mut epsilon: String = "".to_string();
    let v = read_a_file(File::open("input3")?)?;
    let line_length = v[0].len();
    let mut n:usize = 0;
    while n < line_length {
        if(majority_zero(&v, n)){
            gamma.push("0".parse().unwrap());
            epsilon.push("1".parse().unwrap());
        }else {
            gamma.push("1".parse().unwrap());
            epsilon.push("0".parse().unwrap());
        }
        n += 1;
    }
    let gamma_int:usize = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_int:usize = usize::from_str_radix(&epsilon, 2).unwrap();
    println!("Svar 3_1: {}", gamma_int * epsilon_int);
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

fn majority_zero(v:&Vec<String>, n:usize) -> bool {
    let mut counter_zero = 0;
    let mut counter_one = 0;
    let mut majority_zero = false;
    for line in v{
        let char_vec:Vec<_> = line.chars().collect();
        let char = char_vec[n];
        if char.to_string() == "0".to_string() {
            counter_zero += 1;
        } else {
            counter_one += 1;
        }
    }
    if(counter_zero > counter_one) {
        majority_zero = true;
    }
    return majority_zero;
}