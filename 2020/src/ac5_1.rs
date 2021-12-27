#![allow(dead_code)]

use std::fs::File;
use regex::Regex;
use regex::RegexSet;
use array2d::Array2D;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

struct Row {
    x: usize,
    y: usize,
}

struct Column {
    x: usize,
    y: usize,
}

impl Default for Row {
    fn default() -> Self {
        Row {
            x: 0,
            y: 127,
        }
    }
}

impl Default for Column {
    fn default() -> Self {
        Column {
            x: 0,
            y: 7,
        }
    }
}

pub fn ac51() -> Result<(), Error>{
    let v = read_a_file(File::open("input51")?)?;
    let mut seatids:Vec<usize> = Vec::new();
    let mut seatmap = Array2D::filled_with(0, 128, 8);
    for line in v {
        let mut r:Row = Row::default();
        let mut c:Column = Column::default();
        let chars: Vec<char> = line.chars().collect();
        let mut n = 0;
        while n < chars.len(){
            if (n < 7){
                if (chars[n].to_string() == "B"){
                    r.x = (r.x + ((rows_total_left(&r)/2)));
                } else{
                    r.y = (r.y - ((rows_total_left(&r)/2)));
                }
            } else {
                if (chars[n].to_string() == "R"){
                    c.x = (c.x + ((columns_total_left(&c)/2)));
                } else {
                    c.y = (c.y - ((columns_total_left(&c)/2)));
                }
            }
        n += 1;
        }
        let seatid:usize = (r.x * 8) + c.x;
        println!("Row: {}", r.x);
        println!("Column: {}", c.x);
        println!("Seatid: {}", seatid);
        seatids.push(seatid);
        seatmap.set(r.x,c.x,seatid);
    }
    println!("All elements:");
    for row_iter in seatmap.rows_iter() {
        for element in row_iter {
            print!("{} ", element);
        }
        println!();
    }
    let maxValue = seatids.iter().max();
    match maxValue {
        Some(max) => println!("Max seatid: {}", max),
        None             => println!("Vector is empty"),
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

fn rows_total_left(r: &Row) -> usize {
    return ((r.y + 1) - r.x);
}

fn columns_total_left(c: &Column) -> usize {
    return ((c.y + 1) - c.x);
}