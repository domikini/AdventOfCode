#![allow(dead_code)]
use std::fs::File;
use array2d::Array2D;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::ptr::read_unaligned;

pub fn ac11_1() -> Result<(), Error>{
    let v = read_a_file(File::open("input11")?)?;

    //Read all lines from input file and put all lines in a Vec
    let mut all_lines:Vec<Vec<usize>> = Vec::new();
    for l in v {
        let line:Vec<char> = l.chars().collect();
        let line_int:Vec<usize> = line.into_iter().map(|c|(c.to_digit(10).unwrap() as usize)).collect();
        all_lines.push(line_int);
    }

    //Create Array2D map
    let mut map:Array2D<_> = Array2D::from_rows(&all_lines);
    let mut result = MapCounter{ map, counter: 0 };
    let mut flash_counter = 0;
    let mut iteration:usize = 0;
    while iteration < 100 {
        result.map = increase_map_with_one_on_step(&result.map);
        result = increase_map_with_flashes_wrapper(&result.map);
        result.map = reset_all_flashed_fired_to_zero(&result.map);
        flash_counter += result.counter;
        iteration += 1;
    }
    println!("Svar: 11_1: {}", flash_counter);
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

struct Point {
    name:Direction,
    row:i32,
    column:i32,
}

enum Direction{
    UP,
    DOWN,
    LEFT,
    RIGHT,
    UPLEFT,
    DOWNLEFT,
    UPRIGHT,
    DOWNRIGHT,
}

struct MapCounter{
    map:Array2D<usize>,
    counter:usize,
}

fn increase_map_with_flashes_wrapper(map:&Array2D<usize>) -> MapCounter {
    let mut result:MapCounter = MapCounter { map: map.clone(), counter: 0 };
    let mut flash_counter = 0;
    while true {
        result = increase_map_with_flashes(&result.map);
        flash_counter += result.counter;
        if(!map_contains_flashable(&result.map)){
            break;
        }
    }
    result.counter = flash_counter;
    return result;
}

fn increase_map_with_flashes(map:&Array2D<usize>) -> MapCounter {
    let mut result:MapCounter = MapCounter { map: map.clone(), counter: 0 };
    let mut column = 0;
    let mut row= 0;
    while row < map.row_len(){
        column = 0;
        while column < map.column_len(){
            let element = result.map.get(row, column).unwrap();
            if(element >= &(10 as usize) && element < &(9999 as usize)){
               if(row != 0){
                   result.map = point_increment(&result.map, Direction::UP, &row, &column);
               }
               if(row != 0 && column != 0){
                   result.map = point_increment(&result.map, Direction::UPLEFT, &row, &column);
               }
               if(row != 0 && column != (result.map.column_len() - 1)){
                   result.map = point_increment(&result.map, Direction::UPRIGHT, &row, &column);
               }
               if(row != (result.map.row_len() - 1)){
                   result.map = point_increment(&result.map, Direction::DOWN, &row, &column);
               }
                if(row != (result.map.row_len() - 1) && column != 0){
                    result.map = point_increment(&result.map, Direction::DOWNLEFT, &row, &column);
                }
                if(row != (result.map.row_len() - 1) && column != (result.map.column_len() - 1)){
                    result.map = point_increment(&result.map, Direction::DOWNRIGHT, &row, &column);
                }
               if(column != 0){
                   result.map = point_increment(&result.map, Direction::LEFT, &row, &column);
               }
               if(column != result.map.column_len() - 1){
                   result.map = point_increment(&result.map, Direction::RIGHT, &row, &column);
               }
                //Highlight current element as flashed by setting value to 9999
                result.map.set(row,column,9999);
                result.counter += 1;
            }
            column += 1;
        }
        row += 1;
    }
    return result;
}

fn map_contains_flashable(map:&Array2D<usize>) -> bool {
    let map = map.clone();
    let flashable:Vec<usize> = map.as_row_major().into_iter().filter(|e| e >= &(10 as usize) && e < &(9999 as usize)).collect();
    if(flashable.len() > 0) {
        return true
    } else {
        return false
    };
}

fn point_increment(map:&Array2D<usize>, direction: Direction, row:&usize, column:&usize) -> Array2D<usize> {
    let mut map_changed = map.clone();
    let mut current:usize;
    current = *map_changed.get((*row as i32 + direction_in_points(&direction).row) as usize, ((*column as i32 + direction_in_points(&direction).column) as usize)).unwrap();
    map_changed.set((*row as i32 + direction_in_points(&direction).row) as usize, (*column as i32 + direction_in_points(&direction).column) as usize, (current + 1));
    return map_changed;
}

fn direction_in_points(direction: &Direction) -> Point {
    match direction {
        Direction::UP => Point {
            name: Direction::UP,
            row: -1,
            column: 0
        },
        Direction::DOWN => Point {
            name: Direction::DOWN,
            row: 1,
            column: 0
        },
        Direction::LEFT => Point {
            name: Direction::LEFT,
            row: 0,
            column: -1
        },
        Direction::RIGHT => Point {
            name: Direction::RIGHT,
            row: 0,
            column: 1
        },
        Direction::UPLEFT => Point {
            name: Direction::UPLEFT,
            row: -1,
            column: -1
        },
        Direction::DOWNLEFT => Point {
            name: Direction::DOWNLEFT,
            row: 1,
            column: -1
        },
        Direction::UPRIGHT => Point {
            name: Direction::UPRIGHT,
            row: -1,
            column: 1
        },
        Direction::DOWNRIGHT => Point {
            name: Direction::DOWNRIGHT,
            row: 1,
            column: 1
        },
    }
}

fn increase_map_with_one_on_step(map:&Array2D<usize>) -> Array2D<usize> {
    let mut map_vec: Vec<usize> = map.as_row_major();
    map_vec = map_vec.into_iter().map(|mut e| e + 1).collect();
    return Array2D::from_row_major(&map_vec,10, 10);
}

fn reset_all_flashed_fired_to_zero(map:&Array2D<usize>) -> Array2D<usize> {
    let mut map_vec: Vec<usize> = map.as_row_major();
    map_vec = map_vec.into_iter().map(|mut e| if(e >= 9999 ){ 0 } else { e }).collect();
    return Array2D::from_row_major(&map_vec,10, 10);
}