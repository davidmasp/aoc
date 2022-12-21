

//day 1

// read the input file..
// https://stackoverflow.com/questions/13579266/
use std::io;
use std::io::prelude::*;

fn main() {
    // this is the line helper
    let mut current: u32 = 0;
    println!("Hello, world!");
    let mut vec: Vec<u32> = Vec::new();

    // gets the stdin
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // `unwrap` returns a `panic` when it receives a `None`.
        // println!("{}", line.unwrap());
        let this_line = line.unwrap();
        if this_line.is_empty(){
            //println!("empty line");
            vec.push(current);
            current = 0;
        } else {
            current += this_line.parse::<u32>().unwrap();
        }
    }

    // now I need to extract the max value
    vec.sort();
    let max_value = vec[vec.len()-1];
    println!("max value: {}", max_value);
    // and the sum of the 3 max values
    let total_len = vec.len();
    let tail = &vec[total_len-3..];
    let max3: u32 = tail.iter().sum();
    println!("max 3: {}", max3);

}
