#![allow(dead_code)]
use std::borrow::Borrow;
use std::convert::TryInto;
use std::fs::File;
use array2d::Array2D;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::num::NonZeroUsize;

pub fn ac9_1() -> Result<(), Error>{
    let v = read_a_file(File::open("input9")?)?;
    //Read all lines and create rows (Vec) of integers
    let mut rows:Vec<Vec<usize>> = Vec::new();
    for l in v {
        let row:Vec<char> = l.chars().collect();
        let row_int = row.into_iter().map(|v |v.to_digit(10).unwrap() as usize).collect::<Vec<usize>>();
        // println!("{:?}", row_int);
        rows.push(row_int);
    }

    //Create Array2D map from Vec of rows
    let map:Array2D<usize> = Array2D::from_rows(&*rows);
    let mut low_map:Array2D<usize> = Array2D::from_rows(&*rows);
    // println!("{:?}", map);

    let mut row = 0;
    let mut column = 0;
    while row < map.row_len(){
        column = 0;
        while column < map.column_len(){
                let current = map.get(row, column).unwrap();
                let mut up:usize = 0;
                let mut down:usize = 0;
                let mut left:usize = 0;
                let mut right:usize = 0;
                if (row != 0){
                    up = *map.get(row - 1, column).unwrap();
                }
                if (row != map.row_len() - 1){
                    down = *map.get(row + 1, column).unwrap();
                }
                if (column != 0){
                    left = *map.get(row, column - 1).unwrap();
                }
                if (column != map.column_len() - 1){
                   right = *map.get(row, column + 1).unwrap();
                }
                if (current < &up && current < &down && current < &left && current < &right){
                    low_map.set(row, column, 1234567890);
                } else if (row == 0 && current < &down && current < &left && current < &right) {
                    low_map.set(row, column, 1234567890);
                } else if (row == (map.row_len() -1) && current < &up && current < &left && current < &right) {
                    low_map.set(row, column, 1234567890);
                } else if (column == 0 && current < &up && current < &down && current < &right) {
                    low_map.set(row, column, 1234567890);
                } else if (column == (map.column_len() - 1) && current < &up && current < &down && current < &left) {
                    low_map.set(row, column, 1234567890);
                }
        column += 1;
        }
    row += 1;
    }

    let mut sum:usize = 0;
    let mut row = 0;
    let mut column = 0;
    while row < low_map.row_len(){
        column = 0;
        while column < low_map.column_len(){
            if(*low_map.get(row, column).unwrap() == 1234567890){
                sum += (map.get(row, column).unwrap() + 1);
            }
        column += 1;
        }
    row += 1;
    }
    println!("Svar 9_1: {}", sum);
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


