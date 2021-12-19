#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::fs::File;
use array2d::Array2D;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use rand::seq::SliceRandom;

pub fn ac12_1() -> Result<(), Error>{
    let v = read_a_file(File::open("input12")?)?;

    let mut moves:Vec<Move> = Vec::new();

    for l in v {
        let mut new_move:Vec<Move> = create_new_move_from_line(&l);
        moves.append(&mut new_move);
    }

    let mut paths = Vec::new();

    let mut iteration = 0;
    while iteration < 5000000 {
        paths.push(create_path_from_moves(&moves));
        iteration += 1;
    }

    paths = paths.into_iter().filter(|p|p.moves[p.moves.len() - 1].end == "end").collect();

    let mut paths_hashset = HashSet::new();
    for p in paths{
        let mut path_string = String::new();
        for m in p.moves{
            path_string.push_str(&*m.start);
            path_string.push_str(",");
        }
        path_string.push_str("end");
        paths_hashset.insert(path_string);
    }
    println!("Svar 12_1: {}", paths_hashset.len());
    Ok(())
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Move{
    start:String,
    end:String,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Path{
    moves:Vec<Move>
}

fn create_path_from_moves(moves: &Vec<Move>) -> Path {
    let mut path:Path = Path { moves: Vec::new() };

    //Create different move lists. One for possible start moves and the other for all other possible moves.
    let mut start_moves:Vec<Move> = moves.clone();
    start_moves = extract_start_steps(start_moves);

    let mut moves_list_withouth_start:Vec<Move> = moves.clone();
    moves_list_withouth_start = extract_all_steps_except_start_steps(moves_list_withouth_start);

    //Make first step from start randomized from start move list
    let mut first_step = start_moves.choose(&mut rand::thread_rng()).unwrap().clone();
    path.moves.push(first_step.clone());

    //Remove moves if lowercase after first move from start
    moves_list_withouth_start = remove_moves_from_movelist_if_lowercase(&moves_list_withouth_start, &first_step);

    while true {
        if(path.moves[path.moves.len() - 1].end == "end"){
            break;
        }

        //Extract the current step name
        let current_step = &path.moves.get(path.moves.len() - 1).unwrap();

        //Extract all possible moves from current step name
        let mut possible_next_moves:Vec<Move> = extract_next_steps(&moves_list_withouth_start, &current_step);
        let next_move = possible_next_moves.choose(&mut rand::thread_rng());
        if (next_move.is_some()){
            path.moves.push(next_move.unwrap().clone());
        }
        else {
            break;
        }
        moves_list_withouth_start = remove_moves_from_movelist_if_lowercase(&moves_list_withouth_start, &next_move.unwrap());
    }
    return path;
}

fn remove_moves_from_movelist_if_lowercase(moves:&Vec<Move>, current_move: &Move) -> Vec<Move> {
    let mut moves:Vec<Move> = moves.clone();
    if(current_move.end.chars().all(char::is_lowercase)){
        moves.retain(|m|*m.end.to_string() != current_move.end.to_string());
        return moves;
    } else {
        return moves;
    }
}

fn extract_next_steps(moves: &Vec<Move>, current_step:&Move) -> Vec<Move>{
    let moves:Vec<Move> = moves.clone();
    return moves.into_iter().filter(|m|m.start.to_string() == current_step.end.to_string()).collect();
}

fn extract_all_steps_except_start_steps(moves:Vec<Move>) -> Vec<Move> {
    let moves:Vec<Move> = moves.clone();
    return moves.into_iter().filter(|m| m.start != "start").collect();
}

fn extract_start_steps(moves:Vec<Move>) -> Vec<Move> {
    let moves:Vec<Move> = moves.clone();
    return moves.into_iter().filter(|m| m.start == "start").collect();
}


fn create_new_move_from_line(line:&String) -> Vec<Move> {
    let mut new_move:Move = Move { start: "".to_string(), end: "".to_string() };
    let line = line.clone();
    let line_vec:Vec<&str> = line.split("-").collect();
    let start = line_vec[0];
    let end = line_vec[1];
    if (start == "start" || end == "end"){
        return create_new_move(&start, &end, false);
    } else if (end == "start" || start == "end") {
        return create_new_move(&end, &start, false);
    } else {
       return create_new_move(&start, &end, true);
    }
}

fn create_new_move(start:&str, end:&str, reverse:bool) -> Vec<Move> {
    let mut moves:Vec<Move> = Vec::new();
    let new_move:Move = Move { start: start.to_string(), end: end.to_string() };
    moves.push(new_move);
    if (reverse) {
        let reverse_move:Move = Move { start: end.to_string(), end: start.to_string() };
        moves.push(reverse_move);
    }
    return moves;
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