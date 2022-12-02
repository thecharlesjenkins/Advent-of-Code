use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;



fn main() {
    let mut sum = 0;
    let mut second_sum = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(turn) = line {
                let theirs = turn.chars().nth(0).unwrap();
                let mine = turn.chars().nth(2).unwrap();
                sum += points(theirs, mine);
                second_sum += points2(theirs, mine);
            }
        }
    }
    println!("{}", sum);
    println!("{}", second_sum);
}

// A Beats Z (0, 2)
// B Beats X (1, 0)
// C Beats Y (2, 1)
fn points(theirs: char, mine: char) -> i32 {
    let their_ascii = (theirs as i32) - 65; 
    let my_ascii = (mine as i32) - 88;
    let win = (((my_ascii + 2) % 3 == their_ascii)) as i32 * 6;
    let tie = (their_ascii == my_ascii) as i32 * 3;
    return win + tie + my_ascii + 1;
}


// Rock ties Rock
// Rock loses Paper
// Rock beats Scissors

// 0 ties 0
// 0 loses 1
// 0 wins 2
// X select lose (0)
// Y select tie (1)
// Z select win (2)
// 0 -> 2
// 1 -> 0
// 2 -> 1
fn points2(theirs: char, mine: char) -> i32 {
    let their_ascii = (theirs as i32) - 65; 
    let my_ascii = (their_ascii + (mine as i32 - 88) + 2) % 3;
    return my_ascii + 1 + (mine as i32 - 88) * 3;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}