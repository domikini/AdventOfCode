#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::fs::File;
use array2d::Array2D;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::iter::FromIterator;
use std::thread;
use rand::seq::SliceRandom;

pub fn ac12_2() -> Result<(), Error> {
    let v = read_a_file(File::open("input12")?)?;

    let mut moves: Vec<Move> = Vec::new();

    for l in v {
        let mut new_move: Vec<Move> = create_new_move_from_line(&l);
        moves.append(&mut new_move);
    }

    let mut paths = Vec::new();

    let mut inner_iteration = 0;
    let mut outer_iteration = 1;
    let mut paths_hashset = HashSet::new();

    while outer_iteration < 6 {

        let mut handles= Vec::new();
        inner_iteration = 0;
        while inner_iteration < 1000000 {
            let moves_clone = moves.clone();
            let handle = thread::spawn(move || {
                return create_path_from_moves(moves_clone);
            });
            handles.push(handle);
            if (inner_iteration % 10000 == 0) {
                println!("{}", outer_iteration * inner_iteration);
            }
            inner_iteration += 1;
        }

        for handle in handles {
            paths.push(handle.join().unwrap());
        }

        paths = paths.into_iter().filter(|p| p.moves[p.moves.len() - 1].end == "end").collect();

        for p in &paths {
            let mut path_string = String::new();
            for m in &p.moves {
                path_string.push_str(&*m.start);
                path_string.push_str(",");
            }
            path_string.push_str("end");
            paths_hashset.insert(path_string);
        }


        for p in &paths_hashset{
            println!("{:?}", p);
        }
        outer_iteration += 1;
    }


    println!("Svar 12_2: {}", paths_hashset.len());
    Ok(())
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Move{
    start:String,
    end:String,
    special_small_cave:bool,
    special_small_cave_iteration:usize,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Path{
    moves:Vec<Move>
}

fn create_path_from_moves(moves: Vec<Move>) -> Path {
    let mut path:Path = Path { moves: Vec::new() };

    //Randomly pick one small cave as special cave that can be visited twice
    let moves_picked_special_small_cave = randomly_pick_small_cave_as_special_cave(moves);

    //Create different move lists. One for possible start moves and the other for all other possible moves.
    let mut start_moves:Vec<Move> = moves_picked_special_small_cave.clone();
    start_moves = extract_start_steps(start_moves);

    let mut moves_list_withouth_start:Vec<Move> = moves_picked_special_small_cave.clone();
    moves_list_withouth_start = extract_all_steps_except_start_steps(moves_list_withouth_start);

    //Make first step from start randomized from start move list
    let mut first_step = start_moves.choose(&mut rand::thread_rng()).unwrap().clone();
    path.moves.push(first_step.clone());


    if (first_step.special_small_cave == true) {
        moves_list_withouth_start = increase_small_iteration_with_one( moves_list_withouth_start, &first_step.end);
    } else {
        //Remove moves if lowercase after first move from start
        moves_list_withouth_start = remove_moves_from_movelist_if_lowercase(&moves_list_withouth_start, &first_step);
    }


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

        if ((next_move.unwrap().special_small_cave == true && next_move.unwrap().special_small_cave_iteration == 0)) {
            moves_list_withouth_start = increase_small_iteration_with_one( moves_list_withouth_start, &next_move.unwrap().end);
        } else {
        moves_list_withouth_start = remove_moves_from_movelist_if_lowercase(&moves_list_withouth_start, &next_move.unwrap());
        }
    }
    return path;
}

fn increase_small_iteration_with_one(moves:Vec<Move>, special_cave:&String) -> Vec<Move> {
    let moves:Vec<Move> = moves.clone();
    let mut moves_changed = Vec::new();
    for mut m in moves {
        if (m.end == special_cave.to_string() ) {
            m.special_small_cave_iteration += 1;
        }
        moves_changed.push(m);
    }
    return moves_changed;
}

fn randomly_pick_small_cave_as_special_cave(moves: Vec<Move>) -> Vec<Move> {
    let moves:Vec<Move> = moves.clone();
    let mut list_of_small_caves:Vec<String> = vec!["yi".to_string(), "qc".to_string(), "xx".to_string(), "iy".to_string(), "qe".to_string()];
    let mut special_cave = &list_of_small_caves.choose(&mut rand::thread_rng()).unwrap().clone();

    let mut moves_changed = Vec::new();
    for mut m in moves {
        if (m.end == special_cave.to_string() ) {
            m.special_small_cave = true;
        }
        moves_changed.push(m);
    }
    return moves_changed;
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
    let mut new_move:Move = Move { start: "".to_string(), end: "".to_string() , special_small_cave: false, special_small_cave_iteration: 0};
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
    let new_move:Move = Move { start: start.to_string(), end: end.to_string(), special_small_cave:false, special_small_cave_iteration: 0};
    moves.push(new_move);
    if (reverse) {
        let reverse_move:Move = Move { start: end.to_string(), end: start.to_string(), special_small_cave:false, special_small_cave_iteration: 0 };
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