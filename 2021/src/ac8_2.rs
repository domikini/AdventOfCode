#![allow(dead_code)]

use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fs::File;
use array2d::Array2D;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::ops::Deref;
use std::str::Chars;
use differ::Differ;

pub fn ac8_2() -> Result<(), Error>{
    //Hashmap to convert characters to numbers
    let number_to_usize:HashMap<Number, &str> = HashMap::from([
        (Number::one, "1"),
        (Number::two, "2"),
        (Number::three, "3"),
        (Number::four, "4"),
        (Number::five, "5"),
        (Number::six, "6"),
        (Number::seven, "7"),
        (Number::eight, "8"),
        (Number::nine, "9"),
        (Number::zero, "0"),
    ]);

    let v = read_a_file(File::open("input8")?)?;
    let mut inputvec_list = Vec::new();
    let mut outputvec_list = Vec::new();
    for l in v {
        let mut input = "".to_string();
        let mut input_vec = Vec::new();
        let mut input_vec_string = Vec::new();
        let mut output = "".to_string();
        let mut output_vec = Vec::new();
        let mut output_vec_string = Vec::new();
        let line_vec:Vec<_> = l.split(" | ").collect();
        input = line_vec[0].to_string();
        input_vec = input.split_whitespace().collect();
        output = line_vec[1].to_string();
        output_vec = output.split_whitespace().collect();
        input_vec_string = input_vec.into_iter().map(|i| i.to_string()).collect();
        output_vec_string = output_vec.into_iter().map(|i|i.to_string()).collect();
        inputvec_list.push(input_vec_string);
        outputvec_list.push(output_vec_string);
    }

    let mut iterator = 0;
    let mut total = 0;

    while iterator < inputvec_list.len() {
        let mut digits = create_list_of_digits();
        for input in &inputvec_list[iterator]{
            if (input.len() == 7){
                digits.get_mut("eight").unwrap().combo = input.clone().chars().collect();
            }
            else if(input.len() == 2){
                digits.get_mut("one").unwrap().combo = input.clone().chars().collect();
            }
            else if(input.len() == 4){
                digits.get_mut("four").unwrap().combo = input.clone().chars().collect();
            }
            else if(input.len() == 3){
                digits.get_mut("seven").unwrap().combo = input.clone().chars().collect();
            }
        }

        //Find all segments with lengths 5 and 6
        let mut segments_five = Vec::new();
        let mut segments_six = Vec::new();
        for input in &inputvec_list[iterator]{
            if(input.len() == 5){
                let hs:HashSet<char> = input.clone().chars().collect();
                segments_five.push(hs);
            }
            else if (input.len() == 6) {
                let hs:HashSet<char> = input.clone().chars().collect();
                segments_six.push(hs);
            }
        }

        //Finding number three combo
        for i in 0..segments_five.len() {
            let diff = &segments_five[i] - &digits.get("one").unwrap().combo;
            let vec_char = diff.into_iter().collect::<Vec<char>>();
            if (vec_char.len() == 3){
                digits.get_mut("three").unwrap().combo = segments_five[i].clone();
                segments_five.remove(i);
                break;
            }
        }

        //Finding number nine combo
        for i in 0..segments_six.len() {
            let diff = &segments_six[i] - &digits.get("three").unwrap().combo;
            let vec_char = diff.into_iter().collect::<Vec<char>>();
            if (vec_char.len() == 1){
                digits.get_mut("nine").unwrap().combo = segments_six[i].clone();
                segments_six.remove(i);
                break;
            }
        }

        //Finding number five combo
        for i in 0..segments_five.len() {
            let diff = &segments_five[i] - &(&digits.get("nine").unwrap().combo - &digits.get("three").unwrap().combo);
            let vec_char = diff.into_iter().collect::<Vec<char>>();
            if (vec_char.len() == 4){
                digits.get_mut("five").unwrap().combo = segments_five[i].clone();
                segments_five.remove(i);
                break;
            }
        }

        //Setting number two combo
        digits.get_mut("two").unwrap().combo = segments_five[0].clone();


        //Setting number six combo
        for i in 0..segments_six.len() {
            let diff = &segments_six[i] - &digits.get("one").unwrap().combo;
            let vec_char = diff.into_iter().collect::<Vec<char>>();
            if (vec_char.len() == 5){
                digits.get_mut("six").unwrap().combo = segments_six[i].clone();
                segments_six.remove(i);
                break;
            }
        }

        //Setting number zero combo
        digits.get_mut("zero").unwrap().combo = segments_six[0].clone();

        let mut outputnumber:String = "".to_string();
        let output_vec = outputvec_list[iterator].clone();
        for o in 0..output_vec.len() {
            let hs:HashSet<char> = output_vec[o].chars().collect();
            for d in &digits {
                if (&hs == &d.1.combo) {
                    outputnumber.push_str(number_to_usize.get(&d.1.number).unwrap());
                }
            }
        }
        total += outputnumber.parse::<usize>().unwrap();

        iterator += 1;
    }

    println!("Svar 8_2: {}", total);
    Ok(())
}

fn create_list_of_digits() -> HashMap<&'static str, Digit> {
    let mut digits = HashMap::new();

    let number_one:Digit = Digit{
        number: Number::one,
        combo: HashSet::new(),
    };
    digits.insert("one", number_one);

    let number_two:Digit = Digit{
        number: Number::two,
        combo: HashSet::new(),
    };
    digits.insert("two", number_two);

    let number_three:Digit = Digit{
        number: Number::three,
        combo: HashSet::new(),
    };
    digits.insert("three", number_three);

    let number_four:Digit = Digit{
        number: Number::four,
        combo: HashSet::new(),
    };
    digits.insert("four", number_four);

    let number_five:Digit = Digit{
        number: Number::five,
        combo: HashSet::new(),
    };
    digits.insert("five", number_five);

    let number_six:Digit = Digit{
        number: Number::six,
        combo: HashSet::new(),
    };
    digits.insert("six", number_six);

    let number_seven:Digit = Digit{
        number: Number::seven,
        combo: HashSet::new(),
    };
    digits.insert("seven", number_seven);

    let number_eight:Digit = Digit{
        number: Number::eight,
        combo: HashSet::new(),
    };
    digits.insert("eight", number_eight);

    let number_nine:Digit = Digit{
        number: Number::nine,
        combo: HashSet::new(),
    };
    digits.insert("nine", number_nine);

    let number_zero:Digit = Digit{
        number: Number::zero,
        combo: HashSet::new(),
    };
    digits.insert("zero", number_zero);

    return digits;
}


#[derive(Debug)]
struct Digit{
    number:Number,
    combo:HashSet<char>,
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum Number {
    one,
    two,
    three,
    four,
    five,
    six,
    seven,
    eight,
    nine,
    zero
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
