use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

fn main() {
    let v: Vec<i32> = import_data("./src/input.txt").unwrap();
    let mut r: Vec<i32> = Vec::new();
    
    for i in &v {
        for j in &v {
            if j + i == 2020 {
                println!("Match: {} {}", i, j);
                r.push(*j);
            }
        }
    }

    println!("{}", r[0]*r[1]);
}

fn import_data(path: &str) -> Result<Vec<i32>, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);

    let mut v: Vec<i32> = Vec::new();

    for line in br.lines() {
        let line = line?;
        let n = line
            .trim()
            .parse::<i32>()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        v.push(n);
    }
    Ok(v)
}
