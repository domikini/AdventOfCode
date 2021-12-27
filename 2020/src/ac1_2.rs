#![allow(dead_code)]
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

pub fn ac1_2() -> Result<(), Error>{
    let v = read_a_file(File::open("input1")?)?;
    let mut n:usize = 0;
    let mut increase_counter:usize = 0;
    while n < (v.len() - 3) {
        if ((v[n+1] + v[n+2] + v[n+3]) > (v[n] + v[n+1] + v[n+2])) {
            increase_counter += 1;
        }
        n += 1;
    }
    println!("Svar 1_2: {}", increase_counter);
    Ok(())
}

fn read_a_file<R: Read>(io: R) -> Result<Vec<u64>, Error>  {
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