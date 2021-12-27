#![allow(dead_code)]

use std::fs::File;
use regex::Regex;
use regex::RegexSet;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

struct Field {
    name: String,
    required: bool,
}

struct Passport {
    byr: Field,
    iyr: Field,
    eyr: Field,
    hgt: Field,
    hcl: Field,
    ecl: Field,
    pid: Field,
    cid: Field,
}

impl Default for Passport {
    fn default() -> Self {
        Passport {
            byr: Field { name: "".to_string(), required: true },
            iyr: Field { name: "".to_string(), required: true },
            eyr: Field { name: "".to_string(), required: true },
            hgt: Field { name: "".to_string(), required: true },
            hcl: Field { name: "".to_string(), required: true },
            ecl: Field { name: "".to_string(), required: true },
            pid: Field { name: "".to_string(), required: true },
            cid: Field { name: "".to_string(), required: false },
        }
    }
}

pub fn ac41() -> Result<(), Error>{
    let mut passport_list:Vec<Passport> = Vec::new();
    let mut p:Passport = Passport::default();
    let v = read_a_file(File::open("input41")?)?;
    let mut n = 0;
    while n < v.len() {
        let mut line = &v[n];
        if(line == ""){
            passport_list.push(p);
            p = Passport::default();
        }
        else{
            let options_vec:Vec<&str> = line.split(" ").collect();
            for option in options_vec {
                let option_pair:Vec<&str> = option.split(":").collect();
                let option_name = option_pair[0];
                let option_value =  option_pair[1];
                match  option_name {
                    "byr" => p.byr.name = option_value.to_string(),
                    "iyr" => p.iyr.name = option_value.to_string(),
                    "eyr" => p.eyr.name = option_value.to_string(),
                    "hgt" => p.hgt.name = option_value.to_string(),
                    "hcl" => p.hcl.name = option_value.to_string(),
                    "ecl" => p.ecl.name = option_value.to_string(),
                    "pid" => p.pid.name = option_value.to_string(),
                    "cid" => p.cid.name = option_value.to_string(),
                    &_ => {},
                }
            }
        }
        n += 1;
    }
    passport_list.push(p);
    print!("{}\n", passport_list.len());
    let v = passport_list.into_iter().filter( |i|is_valid_passport(i)).collect::<Vec<_>>();
    print!("{}", v.len());
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

fn is_valid_passport(p: &Passport) -> bool {
    return p.byr.name != ""
        && is_valid_byr(&p.byr.name)
        && p.iyr.name != ""
        && is_valid_iyr(&p.iyr.name)
        && p.eyr.name != ""
        && is_valid_eyr(&p.eyr.name)
        && p.hgt.name != ""
        && is_valid_height(&p.hgt.name)
        && p.hcl.name != ""
        && is_valid_hair_color(&p.hcl.name)
        && p.ecl.name != ""
        && is_valid_eye_color(&p.ecl.name)
        && p.pid.name != ""
        && is_valid_passportnumber(&p.pid.name);
}

fn is_valid_byr(byr: &String) -> bool {
    let mut is_valid_byr = false;
    let byr_re = Regex::new(r"^\d{4}").unwrap();
    let byr_times = byr_re.find_iter(&byr).count();
    if(byr_times == 1 && byr >= &1920.to_string() && byr <= &2002.to_string()){
        is_valid_byr = true;
    }
    return is_valid_byr;
}

fn is_valid_iyr(iyr: &String) -> bool {
    let mut is_valid_iyr = false;
    let iyr_re = Regex::new(r"^\d{4}").unwrap();
    let iyr_times = iyr_re.find_iter(&iyr).count();
    if(iyr_times == 1 && iyr >= &2010.to_string() && iyr <= &2020.to_string()){
        is_valid_iyr = true;
    }
    return is_valid_iyr;
}

fn is_valid_eyr(eyr: &String) -> bool {
    let mut is_valid_eyr = false;
    let eyr_re = Regex::new(r"^\d{4}").unwrap();
    let eyr_times = eyr_re.find_iter(&eyr).count();
    if(eyr_times == 1 && eyr >= &2020.to_string() && eyr <= &2030.to_string()){
        is_valid_eyr = true;
    }
    return is_valid_eyr;
}

fn is_valid_height(height: &String) -> bool{
    let mut is_valid_height: bool = false;
    let in_re = Regex::new(r"^\d*in").unwrap();
    let in_times = in_re.find_iter(&height).count();
    let cm_re = Regex::new(r"^\d*cm").unwrap();
    let cm_times = cm_re.find_iter(&height).count();
    if(in_times == 1){
        let digits_re = Regex::new(r"(^\d*)").unwrap();
        let digits = digits_re.captures(&height).unwrap();
        if (digits[0] >= *59.to_string() && digits[0] <= *76.to_string()) { is_valid_height = true };
    }if (cm_times == 1) {
        let in_re = Regex::new(r"^\d*").unwrap();
        let digits = in_re.captures(&height).unwrap();
        if (digits[0] >= *150.to_string() && digits[0] <= *193.to_string()) { is_valid_height = true };
    }
    return is_valid_height;
}

fn is_valid_hair_color(hcl: &String) -> bool{
    let mut is_valid_hair_color: bool = false;
    let hcl_re = Regex::new(r"^#[0-9a-f]{6}").unwrap();
    let hcl_times = hcl_re.find_iter(&hcl).count();
    if (hcl_times == 1){
        is_valid_hair_color = true;
    }
    return is_valid_hair_color;
}

fn is_valid_passportnumber(pid: &String) -> bool{
    let mut is_valid_passportnumber: bool = false;
    let pid_re = Regex::new(r"^\d{9}").unwrap();
    let pid_times = pid_re.find_iter(&pid).count();
    if (pid_times == 1){
        is_valid_passportnumber = true;
    }
    return is_valid_passportnumber;
}

fn is_valid_eye_color(ecl: &String) -> bool{
    let mut is_valid_eye_color: bool = false;
    let ecl_re_set = RegexSet::new(&[
        r"^amb",
        r"^blu",
        r"^brn",
        r"^gry",
        r"^grn",
        r"^hzl",
        r"^oth",
    ]).unwrap();
    if (ecl_re_set.is_match(&ecl)){
        is_valid_eye_color = true;
    }
    return is_valid_eye_color;
}