#![allow(dead_code)]

use std::borrow::Borrow;
use std::convert::TryInto;
use std::fs::File;
use array2d::Array2D;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

pub fn ac5_1() -> Result<(), Error>{
    let v = read_a_file(File::open("input5")?)?;

    //Read all lines in input and extract start and end coordinates
    //Extend start and end coordinates with points in between
    let mut lines = Vec::new();
    for l in v {
        let points = coordinates_to_line_vec(&l);
        lines.push(points);
    }
    //Filter so only vertical and horizontal lines remains
    lines = lines.into_iter().filter(|p| !p.is_empty() ).collect();

    //Calculate maximum value of x and y for map creation in next step
    let x_max = lines.iter().flatten().max_by_key(|p|p.x).unwrap().x;
    let y_max = lines.iter().flatten().max_by_key(|p|p.y).unwrap().y;

    //Create map with Array2D
    let mut map:Array2D<usize> = Array2D::filled_with(0, y_max + 1, x_max + 1);

    //Plot all line values to map
    for line in lines{
        map = plot_on_map(&map, &line);
    }

    let overlap_more_than_2 = map.as_row_major().into_iter().filter(|u| u.clone() >= 2 as usize ).collect::<Vec<usize>>().len();

    println!("Svar 5_1: {}", overlap_more_than_2);
    Ok(())
}

fn plot_on_map(map:&Array2D<usize>, line:&Vec<Point>) -> Array2D<usize> {
    let mut map = map.clone();
    let line = line.clone();
    for p in line {
        map.set(p.y, p.x, map.get(p.y, p.x).unwrap() + 1);
    }
    return map;
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
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

fn coordinates_to_line_vec (l: &String) -> Vec<Point> {
    let mut points:Vec<Point> = Vec::new();
    let line_split:Vec<&str> = l.split(" -> ").collect();
    let start:String = line_split[0].to_string();
    let end:String = line_split[1].to_string();
    let start_point: Point = convert_string_to_point(&start);
    let end_point: Point = convert_string_to_point(&end);
    if (start_point.x == end_point.x) {
        // println!("vertical line");
        // println!("start: {} end: {}", start_point.y, end_point.y);
        points = points_generator(extend_endpoints(&start_point.y, &end_point.y), &start_point.x, "x");
    } else if (start_point.y == end_point.y) {
        // println!("horizontal line");
        // println!("start: {} end: {}", start_point.x, end_point.x);
        points = points_generator(extend_endpoints(&start_point.x, &end_point.x), &start_point.y, "y");
    }
    // println!("{:?}", points);
    return points;
}

fn points_generator (endpoints:Vec<usize>, constant:&usize, constant_field:&str) -> Vec<Point> {
    let mut points:Vec<Point> = Vec::new();
    let constant:usize = constant.clone();
    for e in endpoints{
        if(constant_field == "x"){
            points.push(Point { x: constant, y: e });
        }else if (constant_field == "y"){
            points.push(Point { x: e, y: constant });
        }
    }
    return points;
}

fn convert_string_to_point (s: &String) -> Point {
    let string_split:Vec<&str> = s.split(",").collect();
    let x:usize = string_split[0].parse().unwrap();
    let y:usize = string_split[1].parse().unwrap();
    return Point { x: x, y: y };
}

fn extend_endpoints(start:&usize, end:&usize) -> Vec<usize> {
    let start_point = start.clone();
    let end_point = end.clone();
    let mut extended_points:Vec<usize> = Vec::new();
    if (end_point > start_point){
        let diff = end_point - start_point;
        extended_points = generate_number_between_points(&diff, &start_point);
    } else if (start_point > end_point) {
        let diff = start_point - end_point;
        extended_points = generate_number_between_points(&diff, &end_point);
    }
    return extended_points;
}

fn generate_number_between_points(diff:&usize, start_number:&usize) -> Vec<usize> {
    let mut start_number:usize = start_number.clone();
    let mut generated_numbers:Vec<usize> = Vec::new();
    let mut counter = 0;
    while counter < diff + 1 {
        generated_numbers.push(start_number);
        start_number += 1;
        counter += 1;
    }
    // println!("{:?}", generated_numbers);
    return generated_numbers;
}