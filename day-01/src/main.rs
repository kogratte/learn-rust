use day_01::implem::rotate;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn parse_file() -> io::Result<i32> {
    let file = File::open("./input.txt")?;
    let lines = BufReader::new(file).lines();

    let mut counter = 50;
    let mut zeros = 0;

    for line in lines.map_while(Result::ok) {
        let direction = line.chars().next().unwrap_or('?');
        let offset: i32 = line[1..].parse().unwrap_or(0);

        counter = rotate(counter, direction, offset);
        if counter == 0 {
            zeros += 1;
        }
    }

    Ok(zeros)
}

fn main() {
    match parse_file() {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}