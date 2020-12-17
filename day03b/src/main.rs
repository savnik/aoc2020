use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() {
    let map = import_data("./src/input.txt").unwrap();

    let res = [
        navigation(&map, 1, 1),
        navigation(&map, 3, 1),
        navigation(&map, 5, 1),
        navigation(&map, 7, 1),
        navigation(&map, 1, 2),
    ];

    let result = res.iter().fold(1, |acc, x| acc * x);

    println!("result: {} - {:?}", result, res)

}

fn navigation(map: &Vec<Vec<char>>, move_x: usize, move_y: usize) -> i64 {
    let map_width = map[0].len();
    let map_height = map.len();

    let mut x: usize = 0;
    let mut y: usize = 0;

    let mut count_trees = 0;

    while y < map_height-1 {
        // Move
        x += move_x;
        if x >= map_width { // Make x axsis repeat it self.
            x -= map_width
        }
        y += move_y;
        if y >= map_height {
            break;
        }
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
    count_trees
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