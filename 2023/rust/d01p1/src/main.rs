
//day 1

// read the input file..
// https://stackoverflow.com/questions/13579266/
use std::io;
use std::io::prelude::*;

fn main() {
    // this is the line helper
    let mut current_sum: u32 = 0;

    // gets the stdin
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let mut first_digit = 0;
        let mut last_digit = 0;
        let this_line = line.unwrap();

        for c in this_line.chars() {
            let digit_result = c.to_digit(10);

            let digit_value = match digit_result {
                Some(d) => d,
                None => {
                    //println!("not a digit");
                    continue;
                },
            };
            if first_digit == 0 {
                first_digit = digit_value;
            } 
            last_digit = digit_value;       
        };

        let two_dig = format!("{}{}", first_digit, last_digit);
        println!("two dig: {}", two_dig);
        current_sum += two_dig.parse::<u32>().unwrap();
    }
    
    println!("sum: {}", current_sum);

}

