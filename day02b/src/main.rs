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

        let pos1 = split[0].trim().parse::<usize>().map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        let pos2 = split[1].trim().parse::<usize>().map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        let c = split[2].chars().nth(0).expect("Missing char");
        let password = split[3];

        if password.chars().count() >= pos1 && password.chars().count() >= pos2 {
            let char_pos_1 = password.chars().nth(pos1-1).expect("bad pos1");
            let char_pos_2 = password.chars().nth(pos2-1).expect("bad pos2");
            
            if (char_pos_1 == c) != (char_pos_2 == c) {
                count_valid += 1;
            } 
        } 
    }
    println!("Count valid passwords: {}", count_valid);
    Ok(())
}
