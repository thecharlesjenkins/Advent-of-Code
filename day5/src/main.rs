use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    // let mut points = 0;
    if let Ok(lines) = read_lines("/home/charlie/projects/advent_of_code/Advent-of-Code/day5/src/input.txt") {
        let mut first = vec!['J','H','G','M','Z','N','T','F'];
        let mut second = vec!['V','W','J'];
        let mut third = vec!['G','V','L','J','B','T','H'];
        let mut fourth = vec!['B','P','J','N','C','D','V','L'];
        let mut fifth = vec!['F','W','S','M','P','R','G'];
        let mut sixth = vec!['G','H','C','F','B','N','V','M'];
        let mut seventh = vec!['D','H','G','M','R'];
        let mut eighth = vec!['H','N','M','V','Z','D'];
        let mut ninth = vec!['G','N','F','H'];
        let mut inputs = [
            first,
            second,
            third,
            fourth,
            fifth,
            sixth,
            seventh,
            eighth,
            ninth,
        ];
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        for line in lines.skip(10) {
            if let Ok(pairing) = line {
                println!("{}", pairing);
                let text_match = re.captures_iter(&pairing).next().unwrap();
                let number = text_match[1].parse::<usize>().unwrap();
                let origin = text_match[2].parse::<usize>().unwrap();
                let dest = text_match[3].parse::<usize>().unwrap();


                let mut intermediate = Vec::new();
                for _ in 0..number {
                    let res = inputs[origin-1].pop().unwrap();
                    intermediate.insert(0, res);
                }
                inputs[dest-1].append(&mut intermediate);
            }
        }
        for arr in inputs {
            println!("{}", arr.last().unwrap());
        }
        print!("Done\n")
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
