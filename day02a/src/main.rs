use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use regex::Regex;

fn main() {
    println!("Hello, world!");
    import_data("./src/input.txt").unwrap();
}


fn import_data(path: &str) -> Result<(), Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);
    let mut count_valid = 0;
    
    for line in br.lines() {
        let line = line?;
        let split: Vec<&str> = Regex::new(r"(-)|(: )|( )").unwrap().split(&line).collect();

        let min = split[0].trim().parse::<usize>().map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        let max = split[1].trim().parse::<usize>().map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        let c = split[2];
        let password = split[3];
        let count = password.matches(c).count();

        

        if count >= min && count <= max {
            count_valid += 1;
        }

       

       println!("Line: {}, {}, {}, {}, {}", min, max, c, password, count > min && count < max);
    }
    println!("Count valid passwords: {}", count_valid);
    Ok(())
}
