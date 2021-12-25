#![allow(dead_code)]

use std::borrow::Borrow;
use std::convert::TryInto;
use std::fs::File;
use array2d::Array2D;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

pub fn ac6_2() -> Result<(), Error>{
    let v = read_a_file(File::open("input6")?)?;

    let mut counter = Counter{
        zero: 0,
        one: 0,
        two: 0,
        three: 0,
        four: 0,
        five: 0,
        six: 0,
        seven: 0,
        eight: 0
    };

    let mut input:String = "".to_string();
    for l in v {
        input = l;
    }
    let input_list:Vec<_> = input.split(",").collect();

    let mut lanternfish_list = Vec::new();
    for i in input_list{
        lanternfish_list.push(Lanternfish{ counter: i.parse().unwrap() });
    }

    counter.zero = lanternfish_list.iter().filter(|l|l.counter == 0).count();
    counter.one = lanternfish_list.iter().filter(|l|l.counter == 1).count();
    counter.two = lanternfish_list.iter().filter(|l|l.counter == 2).count();
    counter.three = lanternfish_list.iter().filter(|l|l.counter == 3).count();
    counter.four = lanternfish_list.iter().filter(|l|l.counter == 4).count();
    counter.five = lanternfish_list.iter().filter(|l|l.counter == 5).count();
    counter.six = lanternfish_list.iter().filter(|l|l.counter == 6).count();
    counter.seven = lanternfish_list.iter().filter(|l|l.counter == 7).count();
    counter.eight = lanternfish_list.iter().filter(|l|l.counter == 8).count();

    let mut day_iterator = 0;
    while day_iterator < 256 {
        let current_zero = counter.zero;
        let current_one = counter.one;
        let current_two = counter.two;
        let current_three = counter.three;
        let current_four = counter.four;
        let current_five = counter.five;
        let current_six = counter.six;
        let current_seven = counter.seven;
        let current_eight = counter.eight;
        counter.zero = current_one;
        counter.one = current_two;
        counter.two = current_three;
        counter.three = current_four;
        counter.four = current_five;
        counter.five = current_six;
        counter.six = current_seven + current_zero;
        counter.seven = current_eight;
        counter.eight = current_zero;
        day_iterator += 1;
    }

    println!("Svar 6_2: {:?}", counter.zero + counter.one + counter.two + counter.three + counter.four + counter.five + counter.six + counter.seven + counter.eight);
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
struct Counter{
    zero:usize,
    one:usize,
    two:usize,
    three:usize,
    four:usize,
    five:usize,
    six:usize,
    seven:usize,
    eight:usize,
}


#[derive(Debug)]
struct Lanternfish{
    counter:i32,
}
