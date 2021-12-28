#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::fs::File;
use array2d::Array2D;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use rand::seq::SliceRandom;

pub fn ac14_1() -> Result<(), Error>{
    let v = read_a_file(File::open("input14")?)?;
    let start_seq = v[0].clone();
    let mut pair_ins_map:HashMap<String, String> = HashMap::new();

    for i in 2..v.len() {
        let line_split:Vec<&str> = v[i].split(" -> ").collect();
        pair_ins_map.insert(line_split[0].to_string(), line_split[1].to_string());
    }

    //Extract keys
    let mut keys_vec:Vec<&String> = pair_ins_map.keys().collect();
    let mut char_vec:Vec<char> = start_seq.chars().collect();

    for key in keys_vec{
        let chars= key.chars().collect::<Vec<char>>();
        for c in chars{
            char_vec.push(c);
        }
    }
    char_vec.sort();
    char_vec.dedup();

    let mut char_tuple:Vec<_> = Vec::new();
    let mut tmp = [0; 4];
    for c in char_vec{
        let pair = (c.to_string(), 0 as usize);
        char_tuple.push(pair);
    }

    let mut counter_map:HashMap<String, usize> = char_tuple.into_iter().collect();

    let mut seq_vec:Vec<char> = start_seq.chars().collect::<Vec<char>>();
    let mut new_seq_vec:Vec<char> = seq_vec.clone();
    let mut iterator = 0;
    while iterator < 10{
        for i in 0..seq_vec.len() - 1 {
            let mut pair:String = "".to_string();
            pair.push(seq_vec[i]);
            pair.push(seq_vec[i+1]);
            let insert_char = pair_ins_map.get(pair.as_str()).unwrap().clone();
            new_seq_vec.insert(2*i+1, insert_char.parse().unwrap());
        }
        seq_vec = new_seq_vec.clone();
        iterator += 1;
    }

    for c in &seq_vec {
        *counter_map.get_mut(&c.to_string()).unwrap() += 1;
    }

    //Extract max and min occurences
    let max= counter_map.iter().max_by_key(|&(_,value)|value).unwrap();
    let min = counter_map.iter().min_by_key(|&(_,value)|value).unwrap();

    println!("Svar 14_1: {:?}", max.1 - min.1);
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