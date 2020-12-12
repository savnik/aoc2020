use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() {
    let map = import_data("./src/input.txt").unwrap();
    let map_width = map[0].len();
    let map_height = map.len();
    println!("Width: {}, Height: {}", map_width, map_height);

    let mut x: usize = 0;
    let mut y: usize = 0;
    let move_x: usize = 3;
    let move_y: usize = 1;

    let mut count_trees = 0;

    while y < map_height-1 {
        // Move
        x += move_x;
        if x >= map_width { // Make x axsis repeat it self.
            x -= map_width
        }
        y += move_y;
        //println!("Position {}, {}", x, y);
        //println!("{:?}", map[y]);

        // Get position
        let pos = map[y][x];
        println!("{}", pos);
        if pos == '#' {
            count_trees += 1;
            println!("Tree hit!");
        }
    }

    println!("Total hits: {}", count_trees);

}


fn import_data(path: &str) -> Result<Vec<Vec<char>>, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);
    let mut map: Vec<Vec<char>> = Vec::new();
    
    for line in br.lines() {
        let line = line?;
        let char_vec: Vec<char> = line.chars().collect();
        map.push(char_vec);
    }
    Ok(map)
}