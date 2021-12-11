#![allow(dead_code)]
use std::borrow::Borrow;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use array2d::Array2D;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::num::NonZeroUsize;
use regex::Regex;
use crate::ac10_1::CorruptStatus::corrupt;

pub fn ac10_1() -> Result<(), Error>{
    let v = read_a_file(File::open("input10")?)?;

    //Hashmap to convert characters to be represented by numbers
    let chars_to_usize = HashMap::from([
        ("(", 2),
        (")", -2),
        ("[", 3),
        ("]", -3),
        ("{", 5,),
        ("}", -5),
        ("<", 7),
        (">", -7),
    ]);

    //Hashmap to convert characters to syntax error score
    let chars_to_syntax_error_points = HashMap::from([
        (")", 3),
        ("]", 57),
        ("}", 1197),
        (">", 25137),
    ]);

    let mut all_lines:Vec<Line> = Vec::new();

    //Convert all characters in each line to numbers according to Hashmap
    for l in v {
        let chars:Vec<char> = l.chars().collect();
        let mut line:Line = Line { chars_as_char: Vec::new(), chars_as_number:Vec::new(), corrupt_status: CorruptStatus::healthy, length: l.len() };
        let mut pos:usize = 0;
        while pos < chars.len() {
            line.chars_as_number.push(chars_to_usize.get(chars[pos].encode_utf8(&mut [0,4])).unwrap().clone());
            line.chars_as_char.push(chars[pos]);
            pos += 1;
        }
        all_lines.push(line);
    }

    //Looping through the numbers in all lines. All positive numbers will be collected into a "debt" list and will be deducted by a negative number. Last number in the debt list must be deducted by an equal negative number. When deducted the last positive number will be removed from the debt list. If the negative number deducts a non equal positive number in the debt list, it will render a mismatch and will cast an exception.
    let mut syntax_error_points_sum:usize = 0;
    for mut line in &mut all_lines {
        let mut debt_list:Vec<i32> = Vec::new();
        'inner: for number in &line.chars_as_number{
            if (number > &0) {
                debt_list.push(*number);
            } else if (number < &0){
                let last_number_in_debt_list = debt_list[debt_list.len() - 1];
                if ( last_number_in_debt_list + number == 0 ) {
                    debt_list.remove(debt_list.len() -1);
                } else {
                    line.corrupt_status = CorruptStatus::corrupt;
                    let last_char = find_key_for_value(&chars_to_usize, &number).unwrap();
                    let last_char_in_debt_list = find_key_for_value(&chars_to_usize, &(&last_number_in_debt_list * -1) ).unwrap();
                    let last_char_syntax_error_score = &chars_to_syntax_error_points.get(last_char).unwrap();
                    syntax_error_points_sum += *last_char_syntax_error_score;
                    // println!("Line is corrupt! Expected {}, but got {}", last_char_in_debt_list, last_char);
                    break 'inner;
                }
            }
        }
    }
    println!("Svar 10_1: {}", syntax_error_points_sum);
    Ok(())
}

#[derive(Debug)]
struct Line {
    chars_as_char:Vec<char>,
    chars_as_number:Vec<i32>,
    corrupt_status:CorruptStatus,
    length:usize,
}

#[derive(Debug)]
enum CorruptStatus {
    corrupt,
    healthy,
}

fn find_key_for_value<'a>(map: &'a HashMap<&'static str, i32>, value: &i32) -> Option<&'static str> {
    map.iter()
        .find_map(|(&key, &val)| if val == *value { Some(key) } else { None })
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


