use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut max: i32 = 0;
    let mut second: i32 = 0;
    let mut third: i32 = 0;
    let mut current: i32 = 0;
    if let Ok(lines) = read_lines("./part1.txt") {
        for line in lines {
            if let Ok(number) = line {
                if number.eq("") {
                    third = std::cmp::max(std::cmp::min(std::cmp::min(max, current), second), third);
                    second = std::cmp::max(std::cmp::min(max, current), second);
                    max = std::cmp::max(max, current);
                    current = 0;
                } else {
                    current += number.parse::<i32>().unwrap();
                }
            }
        }
    }
    println!("{}", max + second + third);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
