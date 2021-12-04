#![allow(dead_code)]
use std::fs::File;
use regex::Regex;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};


pub fn ac3_2() -> Result<(), Error>{
    let mut v = read_a_file(File::open("input31")?)?;
    let mut oxygen = v.clone();
    let mut co2 = v.clone();
    let mut n:usize = 0;
    while oxygen.len() > 1 {
        oxygen = filter_oxygen(oxygen, n);
        n += 1;
    }
    n = 0;
    while co2.len() > 1 {
        co2 = filter_co2(co2, n);
        n += 1;
    }
    let oxygen_int:usize = usize::from_str_radix(&oxygen[0], 2).unwrap();
    let co2_int:usize = usize::from_str_radix(&co2[0], 2).unwrap();
    println!("Svar 3_2: {}", oxygen_int * co2_int);
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

fn filter_oxygen(mut v: Vec<String>, n:usize) -> Vec<String> {
    let mut v_filtered = Vec::new();
    let mut counter_zero = 0;
    let mut counter_one = 0;
    for line in &v{
        let char_vec:Vec<_> = line.chars().collect();
        let char = char_vec[n];
        if char.to_string() == "0".to_string() {
            counter_zero += 1;
        } else {
            counter_one += 1;
        }
    }
    if(counter_zero > counter_one ) {
        v_filtered = v.into_iter().filter(|line| line.chars().nth(n).unwrap() == "0".parse::<char>().unwrap()).collect();
    } else{
        v_filtered = v.into_iter().filter(|line| line.chars().nth(n).unwrap() == "1".parse::<char>().unwrap()).collect();
    }
    return v_filtered;
}

fn filter_co2(mut v: Vec<String>, n:usize) -> Vec<String> {
    let mut v_filtered = Vec::new();
    let mut counter_zero = 0;
    let mut counter_one = 0;
    for line in &v{
        let char_vec:Vec<_> = line.chars().collect();
        let char = char_vec[n];
        if char.to_string() == "0".to_string() {
            counter_zero += 1;
        } else {
            counter_one += 1;
        }
    }
    if(counter_zero < counter_one || counter_zero == counter_one) {
        v_filtered = v.into_iter().filter(|line| line.chars().nth(n).unwrap() == "0".parse::<char>().unwrap()).collect();
    } else{
        v_filtered = v.into_iter().filter(|line| line.chars().nth(n).unwrap() == "1".parse::<char>().unwrap()).collect();
    }
    return v_filtered;
}