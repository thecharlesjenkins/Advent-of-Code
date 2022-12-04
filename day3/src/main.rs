use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn get_points(character: char) -> u32 {
    let num = character as u32;
    if num < 91 {
        return num - 64 + 26;
    } else {
        return num - 96;
    }
}

fn main() {
    // let mut points = 0;
    let mut part2_points = 0;
    if let Ok(lines) = read_lines("/home/charlie/projects/advent_of_code/Advent-of-Code/day3/src/input.txt") {
        let mut line_count = 0;
        let mut part2_first = HashSet::new();
        let mut part2_second= HashSet::new();
        for line in lines {
            // let mut part1 = HashSet::new();
            if line_count % 3 == 0 {
                part2_first.clear();
                part2_second.clear();
            }
            if let Ok(drawer) = line {
                // let char_count = drawer.chars().count();
                for (i, c) in drawer.chars().enumerate() {
                    // if i < (char_count / 2) {
                    //     part1.insert(c);
                    // } else if part1.contains(&c) {
                    //     points += get_points(c);
                    //     break;
                    // }
                    let cycle = line_count % 3;
                    if cycle == 0 {
                        part2_first.insert(c);
                    } else if cycle == 1 {
                        if part2_first.contains(&c) {
                           part2_second.insert(c);
                        };
                    } else if cycle == 2 && part2_second.contains(&c) {
                        part2_points += get_points(c);
                        break;
                    }
                }
            }
            line_count += 1;
        }
    }
    // println!("{}", points);
    println!("{}", part2_points);
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
