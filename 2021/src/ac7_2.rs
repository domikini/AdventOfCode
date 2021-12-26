#![allow(dead_code)]

use std::borrow::Borrow;
use std::convert::TryInto;
use std::fs::File;
use array2d::Array2D;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

pub fn ac7_2() -> Result<(), Error>{
    let v = read_a_file(File::open("input7")?)?;

    let mut input:String = "".to_string();
    for l in v {
        input = l;
    }
    let input_list:Vec<_> = input.split(",").collect();

    let mut crab_list = Vec::new();
    for i in input_list{
        crab_list.push(Crab{ horizontal_pos: i.parse().unwrap(), fuel_to_new_hoz_pos: 0 });
    }

    let max_hoz_pos = crab_list.iter().max_by_key(|c|c.horizontal_pos).unwrap().horizontal_pos;
    let min_hoz_pos = crab_list.iter().min_by_key(|c|c.horizontal_pos).unwrap().horizontal_pos;
    let mut posstats:Vec<PosStats> = Vec::new();

    let mut iterator = min_hoz_pos;

    while iterator < max_hoz_pos + 1 {
        let new_crab_list = calculate_fuel_consumption_to_move(&crab_list, &iterator);
        let mut fuel_consumption_vec = Vec::new();
        for c in new_crab_list {
            fuel_consumption_vec.push(c.fuel_to_new_hoz_pos);
        }
        posstats.push(PosStats{ horizontal_pos: iterator, fuel_consumption: fuel_consumption_vec.iter().sum() });
        iterator += 1;
    }


    println!("Svar 7_2: {:?}", posstats.iter().min_by_key(|p|p.fuel_consumption).unwrap().fuel_consumption);
    Ok(())
}

fn calculate_fuel_consumption_to_move(crab_list:&Vec<Crab>, new_hoz_pos:&usize) -> Vec<Crab>{
    let mut crab_list = crab_list.clone();
    for mut c in &mut crab_list{
        if (c.horizontal_pos > *new_hoz_pos) {
            c.fuel_to_new_hoz_pos = calculate_fuel_per_crab(c.horizontal_pos - new_hoz_pos);
        }
        else {
            c.fuel_to_new_hoz_pos = calculate_fuel_per_crab(new_hoz_pos - c.horizontal_pos);
        }
    }
    return crab_list;
}

fn calculate_fuel_per_crab (pos_diff:usize) -> usize {
    let mut total_fuel:usize = 0;
    let mut iterator = 1;
    while iterator < pos_diff + 1 {
        total_fuel += iterator;
        iterator += 1;
    }
    return total_fuel;
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



#[derive(Debug, Copy, Clone)]
struct Crab{
    horizontal_pos:usize,
    fuel_to_new_hoz_pos:usize,
}

#[derive(Debug)]
struct PosStats{
    horizontal_pos:usize,
    fuel_consumption:usize,
}
