#![allow(dead_code)]

use std::borrow::Borrow;
use std::convert::TryInto;
use std::fs::File;
use array2d::Array2D;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

pub fn ac6_1() -> Result<(), Error>{
    let v = read_a_file(File::open("input6")?)?;

    let mut input:String = "".to_string();
    for l in v {
        input = l;
    }
    let input_list:Vec<_> = input.split(",").collect();

    let mut lanternfish_list = Vec::new();
    for i in input_list{
        lanternfish_list.push(Lanternfish{ counter: i.parse().unwrap() });
    }

    let mut day_iterator = 0;
    while day_iterator < 80 {
        let mut create_new_counter = 0;
        for mut l in &mut lanternfish_list{
            if (l.counter > 0){
                l.counter -= 1;
            } else if (l.counter == 0) {
                l.counter = 6;
                create_new_counter += 1;
            }
        }

        //Add new lanternfishes for this day
        while create_new_counter > 0 {
            lanternfish_list.push(Lanternfish{ counter: 8 });
            create_new_counter -= 1;
        }

        day_iterator += 1;
    }

    println!("Svar 6_1: {:?}", lanternfish_list.len());
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

#[derive(Debug)]
struct Lanternfish{
    counter:i32,
}
