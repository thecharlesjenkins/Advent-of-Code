use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let mut index = 0;
    if let Ok(lines) =
        read_lines("/home/charlie/projects/advent_of_code/Advent-of-Code/day6/src/input.txt")
    {
        for onlyline in lines.skip(index) {
            if let Ok(signal) = onlyline {
                loop {
                    let mut curr = index;

                    let mut items = HashSet::new();
                    let mut all = [' ';14];
                    if index > signal.chars().count() {
                        println!("fail");
                        return;
                    }
                    for character in signal.chars().skip(index) {
                        if items.contains(&character) {
                            break;
                        }
                        items.insert(character);
                        all[curr-index]=character;
                        curr += 1;
                        if curr == index + 14 {
                            println!("{}", curr);
                            return;
                        }
                    }
                    // if incr {
                    index += 1;
                    // }
                }
            }
        }
    }
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
