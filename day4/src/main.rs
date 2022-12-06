use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // let mut points = 0;
    let mut count = 0;
    if let Ok(lines) = read_lines("/home/charlie/projects/advent_of_code/Advent-of-Code/day4/src/input.txt") {
        for line in lines {
            if let Ok(pairing) = line {
                let mut pairs = pairing.split(&[',', '-'][..]);
                let first = [pairs.next().unwrap().parse::<i32>().unwrap(), pairs.next().unwrap().parse::<i32>().unwrap()];
                let second = [pairs.next().unwrap().parse::<i32>().unwrap(), pairs.next().unwrap().parse::<i32>().unwrap()];
                // if (first[1] <= second[1] && first[0] >= second[0]) || (second[1] <= first[1] && second[0] >= first[0]) { original
                    // count += 1;
                // }
                if (first[1] <= second[1] && first[1] >= second[0]) || (first[0] <= second[1] && first[0] >= second[0]) || (second[0] <= first[1] && second[0] >= first[0]) || (second[1] <= first[1] && second[1] >= first[0]) {
                    count += 1;
                }
            }
        }
    }
    print!("{}\n",count)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
