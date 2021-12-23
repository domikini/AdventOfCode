#![allow(dead_code)]
use std::borrow::Borrow;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use array2d::Array2D;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::num::NonZeroUsize;
use regex::Regex;
use crate::ac10_2::CorruptStatus::{corrupt, healthy};

pub fn ac10_2() -> Result<(), Error>{
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

    //Hashmap to convert characters to completion string score
    let chars_to_completion_score = HashMap::from([
        (")", 1),
        ("]", 2),
        ("}", 3),
        (">", 4),
    ]);

    let mut all_lines:Vec<Line> = Vec::new();

    //Convert all characters in each line to numbers according to Hashmap
    for l in v {
        let chars:Vec<char> = l.chars().collect();
        let mut line:Line = Line { chars_as_char: Vec::new(), chars_as_number:Vec::new(), debt_list_as_number: Vec::new(), completion_string_score: 0, corrupt_status: CorruptStatus::healthy, length: l.len() };
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
        'inner: for number in &line.chars_as_number{
            if (number > &0) {
                line.debt_list_as_number.push(*number);
            } else if (number < &0){
                let last_number_in_debt_list = line.debt_list_as_number[line.debt_list_as_number.len() - 1];
                if ( last_number_in_debt_list + number == 0 ) {
                    line.debt_list_as_number.remove(line.debt_list_as_number.len() -1);
                } else {
                    line.corrupt_status = CorruptStatus::corrupt;
                    break 'inner;
                }
            }
        }
    }

    let mut completion_string_scores = Vec::new();

    for mut line in &mut all_lines{
        if(line.corrupt_status == healthy){
            let mut iteration = line.debt_list_as_number.len() - 1;
            while iteration >= 0 {
                let reverse_char_in_debt_list = find_key_for_value(&chars_to_usize, &(line.debt_list_as_number[iteration] * -1)).unwrap();
                let reverse_char_completion_score = chars_to_completion_score.get(reverse_char_in_debt_list).unwrap();
                line.completion_string_score = ((5 * line.completion_string_score) + *reverse_char_completion_score);
                if iteration != 0 { iteration -= 1; } else { break; }
            }
            completion_string_scores.push(line.completion_string_score);
        }
    }

    //Calculate median
    completion_string_scores.sort();
    let mid = completion_string_scores.len() / 2;

    println!("Svar 10_2: {}", completion_string_scores[mid]);
    Ok(())
}

#[derive(Debug)]
struct Line {
    chars_as_char:Vec<char>,
    chars_as_number:Vec<i32>,
    debt_list_as_number:Vec<i32>,
    completion_string_score:usize,
    corrupt_status:CorruptStatus,
    length:usize,
}

#[derive(Debug, PartialEq)]
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


