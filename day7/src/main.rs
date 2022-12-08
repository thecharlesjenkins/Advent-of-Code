use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

struct Directory {
    pub parent: Option<& Directory>
}

fn main() {
    let acummulation = 0;
    let cd = Regex::new(r"$ cd (\w+|..)").unwrap();
    let ls = Regex::new(r"$ ls").unwrap();
    let dir = Regex::new(r"dir (\w+)").unwrap();
    let file = Regex::new(r"(\d+) (\w+)").unwrap();

    let curr = Directory {
        parent: 
    };
    if let Ok(lines) =
        read_lines("/home/charlie/projects/advent_of_code/Advent-of-Code/day7/src/input.txt")
    {
        for output in lines.skip(1) {
            if let Ok(line) = output {
                
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
