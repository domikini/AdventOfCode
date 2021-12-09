#![allow(dead_code)]

use std::borrow::Borrow;
use std::convert::TryInto;
use std::fs::File;
use array2d::Array2D;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use crate::ac4_2::BoolOrBoolAndUsize::BoolAndUsize;

pub fn ac4_2() -> Result<(), Error>{
    let v = read_a_file(File::open("input4")?)?;
    let mut bingo_sequence = &v[0].split(",").collect::<Vec<_>>();
    let mut bingo_sequence_int:Vec<usize> = bingo_sequence.into_iter().map(|number|number.parse().unwrap()).collect();
    let mut boards:Vec<Array2D<String>> = Vec::new();
    let mut board:Array2D<String>= Array2D::filled_with("".to_string(), 5, 5);
    let mut n:usize = 2;
    let mut r:usize = 0;
    while n < v.len() {
        if(v[n] == "".parse::<String>().unwrap()){
            boards.push(board);
            board = Array2D::filled_with("".to_string(),5,5);
            r = 0;
        } else {
            let line_vec = v[n].split_whitespace().collect::<Vec<_>>();
            let mut c:usize = 0;
            while c < line_vec.len() {
                board.set(r,c, line_vec[c].to_string());
                c += 1;
            }
            r += 1;
        }
        n += 1;
    }
    boards.push(board);
    let mut bingo_iteration:usize = 0;
    while bingo_iteration < bingo_sequence_int.len() {
        let boards_copy = boards.clone();
        let non_bingo_boards: Vec<_> = boards_copy.into_iter().filter(|b|!check_bingo(b)).collect();
        if( non_bingo_boards.len() == 1 ){
            let mut bingo_status:bool;
            let mut board = non_bingo_boards.get(0).cloned().unwrap();
            while bingo_iteration < bingo_sequence_int.len(){
                bingo_status = check_bingo(&board);
                if(bingo_status == true){
                    break;
                }else {
                    board = mark_number_on_board(board, bingo_sequence_int[bingo_iteration]);
                }
            }
            let values_not_marked:Vec<String> = board.as_row_major().into_iter().filter(|value| value != "Marked").collect();
            let values_not_marked_int:Vec<usize> = values_not_marked.into_iter().map(|v|v.parse().unwrap()).collect();
            let sum:usize = values_not_marked_int.into_iter().sum();
            println!("Svar 4_2: {}", sum * bingo_sequence_int[bingo_iteration]);
            break;
        }
        boards = mark_number(boards, bingo_sequence_int[bingo_iteration]);
        bingo_iteration += 1;
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

fn mark_number(boards: Vec<Array2D<String>>, number:usize) -> Vec<Array2D<String>> {
    let mut marked_boards = boards.clone();
    let mut b:usize = 0;
    while b < boards.len() {
        let mut r:usize = 0;
        while r < boards[b].row_len() {
            let mut c:usize = 0;
            while c < boards[b].column_len() {
                if (boards[b].get(r,c).unwrap() == &number.to_string()){
                    marked_boards[b].set(r,c, "Marked".to_string());
                };
                c += 1;
            }
        r += 1;
        }
    b += 1;
    }
    return marked_boards;
}

fn mark_number_on_board(board: Array2D<String>, number:usize) -> Array2D<String> {
    let mut marked_board = board.clone();
    let mut r:usize = 0;
    while r < board.row_len() {
        let mut c:usize = 0;
        while c < board.column_len() {
            if (board.get(r,c).unwrap() == &number.to_string()){
                marked_board.set(r,c, "Marked".to_string());
            };
            c += 1;
        }
        r += 1;
    }
    return marked_board;
}

fn check_bingo(board: &Array2D<String>) -> bool {
    let mut is_there_bingo = false;
    let mut column_and_row_no:usize = 0;
    let mut bingo_row:Vec<_> = Vec::new();
    let mut bingo_column:Vec<_> = Vec::new();
    while column_and_row_no < board.row_len() {
        bingo_row = Vec::new();
        bingo_column = Vec::new();
        bingo_row = board.row_iter(column_and_row_no).filter(|&v|v == "Marked").collect();
        bingo_column = board.column_iter(column_and_row_no).filter(|&v|v == "Marked").collect();
        if(bingo_row.len() == 5 || bingo_column.len() == 5){
            is_there_bingo = true;
            break;
        }
        column_and_row_no += 1;
    }
    return is_there_bingo;
}

enum BoolOrBoolAndUsize<'a> {
    Bool(&'a bool),
    BoolAndUsize(&'a bool, usize),
}

fn return_check_bingo(flag: bool, board_iteration: usize) -> BoolOrBoolAndUsize<'static> {
    if flag {
        BoolOrBoolAndUsize::BoolAndUsize(&true, board_iteration)
    } else {
        BoolOrBoolAndUsize::Bool(&false)
    }
}