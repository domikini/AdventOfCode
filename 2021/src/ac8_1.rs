#![allow(dead_code)]

use std::borrow::Borrow;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use array2d::Array2D;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::ops::Deref;

pub fn ac8_1() -> Result<(), Error>{
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

    let digits = create_list_of_digits();

    let mut counter = 0;

    for output in outputvec_list{
        for s in output{
            if (s.len() == digits.get("one").unwrap().segments ||
                s.len() == digits.get("four").unwrap().segments ||
                s.len() == digits.get("seven").unwrap().segments ||
                s.len() == digits.get("eight").unwrap().segments){
                counter += 1;
            }
        }
    }



    println!("Svar 8_1: {}", counter);
    Ok(())
}

fn create_list_of_digits() -> HashMap<&'static str, Digit> {
    let mut digits = HashMap::new();

    let number_one:Digit = Digit{
        number: Number::one,
        combo: "".to_string(),
        segments: 2
    };
    digits.insert("one", number_one);

    let number_two:Digit = Digit{
        number: Number::two,
        combo: "".to_string(),
        segments: 0
    };
    digits.insert("two", number_two);

    let number_three:Digit = Digit{
        number: Number::three,
        combo: "".to_string(),
        segments: 0
    };
    digits.insert("three", number_three);

    let number_four:Digit = Digit{
        number: Number::four,
        combo: "".to_string(),
        segments: 4
    };
    digits.insert("four", number_four);

    let number_five:Digit = Digit{
        number: Number::five,
        combo: "".to_string(),
        segments: 0
    };
    digits.insert("five", number_five);

    let number_six:Digit = Digit{
        number: Number::six,
        combo: "".to_string(),
        segments: 0
    };
    digits.insert("six", number_six);

    let number_seven:Digit = Digit{
        number: Number::seven,
        combo: "".to_string(),
        segments: 3
    };
    digits.insert("seven", number_seven);

    let number_eight:Digit = Digit{
        number: Number::eight,
        combo: "".to_string(),
        segments: 7
    };
    digits.insert("eight", number_eight);

    let number_nine:Digit = Digit{
        number: Number::nine,
        combo: "".to_string(),
        segments: 0
    };
    digits.insert("nine", number_nine);

    let number_zero:Digit = Digit{
        number: Number::zero,
        combo: "".to_string(),
        segments: 0
    };
    digits.insert("zero", number_zero);

    return digits;
}


#[derive(Debug)]
struct Digit{
    number:Number,
    combo:String,
    segments:usize,
}

#[derive(Debug)]
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
