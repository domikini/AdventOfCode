#![allow(dead_code)]
use std::borrow::Borrow;
use std::convert::TryInto;
use std::fs::File;
use array2d::Array2D;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::num::NonZeroUsize;

pub fn ac9_2() -> Result<(), Error>{
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
    let mut map:Array2D<usize> = Array2D::from_rows(&*rows);

    let mut row = 0;
    let mut column = 0;
    let mut basin_tag = 10;
    while row < map.row_len(){
        column = 0;
        while column < map.column_len(){
                let current = *map.get(row, column).unwrap();
                if(current < 9){
                    check_four_directions(&row, &column, &mut map, basin_tag);
                }
        column += 1;
        basin_tag += 1;
        }
    row += 1;
    }

    let max_value = map.as_row_major().into_iter().max_by_key(|p|*p).unwrap();
    let mut iterator = 0;

    //Group into Basin struct and add count for every basin tag. Skip any point with the value 9.
    let mut basins:Vec<Basin> = Vec::new();
    while iterator < (max_value + 1){
        let count = map.as_row_major().iter().filter(|p|p == &&iterator && p != &&9).count();
        basins.push(Basin { tag: iterator, count: count });
        iterator += 1;
    }

    basins = basins.into_iter().filter(|b|b.count > 0).collect();
    basins.sort_by_key(|b|b.count);
    let total = &basins[basins.len() - 1].count * &basins[basins.len() - 2].count * &basins[basins.len() - 3].count;
    println!("Svar 9_2: {}", total);
    Ok(())
}

#[derive(Debug)]
struct Basin {
    tag:usize,
    count:usize,
}

fn check_four_directions(row:&usize, column:&usize, mut map: &mut Array2D<usize>, basin_tag:usize){
    let mut up:usize = 0;
    let mut down:usize = 0;
    let mut left:usize = 0;
    let mut right:usize = 0;
    let current = map.get(*row, *column).unwrap();
    if(*current < 9 as usize){
        map.set(*row, *column, basin_tag);
    }
    if (*row != 0){
        up = *map.get(*row - 1, *column).unwrap();
        if(up < 9){
            check_four_directions(&(*row - 1), &*column, &mut map, basin_tag);
        }
    }
    if (*row != map.row_len() - 1){
        down = *map.get(*row + 1, *column).unwrap();
        if(down < 9){
            check_four_directions(&(*row + 1), &*column, &mut map, basin_tag);
        }
    }
    if (*column != 0){
        left = *map.get(*row, *column - 1).unwrap();
        if(left < 9){
            check_four_directions(&*row, &(*column - 1), &mut map, basin_tag);
        }
    }
    if (*column != map.column_len() - 1){
        right = *map.get(*row, *column + 1).unwrap();
        if(right < 9){
            check_four_directions(&*row, &(*column + 1), &mut map, basin_tag);
        }
    }
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


