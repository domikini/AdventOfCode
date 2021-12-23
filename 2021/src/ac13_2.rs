#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::fs::File;
use array2d::Array2D;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use rand::seq::SliceRandom;

pub fn ac13_2() -> Result<(), Error>{
    let v = read_a_file(File::open("input13")?)?;
    let mut coordinates = Vec::new();

    for l in v {
     coordinates.push(line_to_point(&l));
    }

    //Calculate maximum value of x and y for map creation in next step
    let x_max = coordinates.iter().max_by_key(|p|p.x).unwrap().x;
    let y_max = coordinates.iter().max_by_key(|p|p.y).unwrap().y;

    //Create map with Array2D
    let mut map:Array2D<usize> = Array2D::filled_with(0, y_max + 1, x_max + 1);

    //Plot all line values to map
    map = plot_on_map(&map, &coordinates);

    map = fold_x(&map, &655 );
    map = fold_y(&map, &447 );
    map = fold_x(&map, &327 );
    map = fold_y(&map, &223 );
    map = fold_x(&map, &163 );
    map = fold_y(&map, &111 );
    map = fold_x(&map, &81 );
    map = fold_y(&map, &55 );
    map = fold_x(&map, &40 );
    map = fold_y(&map, &27 );
    map = fold_y(&map, &13 );
    map = fold_y(&map, &6 );

    print_map(&map);
    println!("Svar 13_2: {}", "test");
    Ok(())
}

fn print_map(map:&Array2D<usize>){
    for line in map.as_rows(){
        for p in &line{
            if *p > 0 {
                print!("#");
            }
            else if *p == 0 {
                print!(".");
            }
        }
        println!("  {}", &line.len());
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

fn fold_x (map:&Array2D<usize>, fold_coordinate:&usize) -> Array2D<usize> {
    let mut map = map.clone().as_rows();
    let x_new_length = *fold_coordinate;
    let mut new_map = Array2D::filled_with(0, map.len(), x_new_length);

    let mut iteration_y = 0;
    let mut iteration_x = 0;
    for line in map{
        iteration_x = 0;
        while iteration_x < line.len() {
            if iteration_x < *fold_coordinate {
                new_map.set(iteration_y, iteration_x, line[iteration_x]);
            }
            else if iteration_x > *fold_coordinate {
                let coordinate_x = (*fold_coordinate as i32 + (*fold_coordinate as i32 - iteration_x as i32));
                let new_element = (new_map.get(iteration_y, coordinate_x as usize)).unwrap() + line[iteration_x];
                new_map.set(iteration_y, coordinate_x as usize, new_element);
            }
            iteration_x += 1;
        }
        iteration_y += 1;
    }

    return new_map;
}

fn fold_y (map:&Array2D<usize>, fold_coordinate:&usize) -> Array2D<usize> {
    let mut map = map.clone().as_columns();
    let y_new_length = *fold_coordinate;
    let mut new_map = Array2D::filled_with(0, y_new_length, map.len() );

    let mut iteration_x = 0;
    let mut iteration_y = 0;

    for column in map{
        iteration_y = 0;
        while iteration_y < column.len() {
            if iteration_y < *fold_coordinate {
                new_map.set(iteration_y, iteration_x, column[iteration_y]);
            }
            else if iteration_y > *fold_coordinate {
                let coordinate_y = (*fold_coordinate as i32 + (*fold_coordinate as i32 - iteration_y as i32));
                let new_element = (new_map.get(coordinate_y as usize, iteration_x)).unwrap() + column[iteration_y];
                new_map.set(coordinate_y as usize, iteration_x as usize, new_element);
            }
            iteration_y += 1;
        }
        iteration_x += 1;
    }

    return new_map;
}

fn plot_on_map(map:&Array2D<usize>, coordinates:&Vec<Point>) -> Array2D<usize> {
    let mut map = map.clone();
    let coordinates = coordinates.clone();
    for p in coordinates {
        map.set(p.y, p.x, map.get(p.y, p.x).unwrap() + 1);
    }
    return map;
}

fn line_to_point(line:&String) -> Point {
    let coordinate:Vec<_> = line.split(",").collect();
    return Point { x: coordinate[0].parse::<usize>().unwrap(), y: coordinate[1].parse::<usize>().unwrap() };
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

// fold along x=655
// fold along y=447
// fold along x=327
// fold along y=223
// fold along x=163
// fold along y=111
// fold along x=81
// fold along y=55
// fold along x=40
// fold along y=27
// fold along y=13
// fold along y=6
